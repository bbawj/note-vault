mod obsidian;
mod embedding;

use crate::embedding::EmbeddingRequestBuilderError;
use crate::embedding::EmbeddingRequestBuilder;
use std::error::Error;

use csv::{FromUtf8Error, Writer, Reader, ReaderBuilder, StringRecord};
use embedding::EmbeddingRequest;
use js_sys::JsString;
use log::debug;
use obsidian::Vault;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use crate::embedding::EmbeddingInput;

#[wasm_bindgen]
pub struct ExampleCommand {
    id: JsString,
    name: JsString,
    vault: Vault,
    client: Client,
}

#[wasm_bindgen]
impl ExampleCommand {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> JsString {
        self.id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: &str) {
        self.id = JsString::from(id)
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsString {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: &str) {
        self.name = JsString::from(name)
    }

    pub async fn callback(&self) {
        obsidian::Notice::new(
            format!(
                "Number of markdown files: {}",
                self.vault.getMarkdownFiles().len()
            )
            .as_str(),
        );
        match self.process_files(self.vault.getMarkdownFiles()).await {
            Ok(()) => (),
            Err(e) => debug!("{:?}", e),
        }
    }

    async fn process_files(&self, files: Vec<obsidian::TFile>) -> Result<(), SemanticSearchError> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        for file in files {
            let extracted = self.extract_sections(file).await.unwrap();
            for (file_name, header, body) in extracted {
                wtr.write_record(&[&file_name, &header, &body])?;
            }
        }
        let data = String::from_utf8(wtr.into_inner()?)?;
        let adapter = self.vault.adapter();
        adapter.append(DATA_FILE_PATH.to_string(), data).await?;
        Ok(())
    }
    
    async fn extract_sections(&self, file: obsidian::TFile) -> std::io::Result<Vec<(String, String, String)>> {
        let mut header_to_content: Vec<(String, String, String)> = Vec::new();
        let name = file.name();
        match self.vault.read(file).await {
            Ok(text) => {
                let text = text.as_string().unwrap();
                let mut header = "".to_string();
                let mut body = "".to_string();
                let mut iterator = text.lines().peekable();
                while let Some(line) = iterator.next() {
                    if line.starts_with("##") {
                        header = line.replace("#", "").trim().to_string();
                        header_to_content.push((name.clone(), header.clone(), body.clone()));
                        body.clear();
                    } else {
                        body += line;
                        if iterator.peek().is_none() && header != "" {
                            header_to_content.push((name.clone(), header.clone(), body.clone()));
                        }
                    }
                }
            },
            Err(_) => todo!(),
        }

        Ok(header_to_content)
    }

    async fn create_embedding_request(&self) -> Result<EmbeddingRequest, SemanticSearchError> {
        let input = self.vault.adapter().read(DATA_FILE_PATH.to_string()).await?.as_string().expect("Input csv is not a string");
        let mut reader = ReaderBuilder::new().trim(csv::Trim::All).flexible(false)
            .from_reader(input.as_bytes());
        let records = reader.records().collect::<Result<Vec<StringRecord>, csv::Error>>()?;
        let string_records = records.iter().map(|record| 
                           record.get(2).unwrap().to_string()
                          ).collect();
        let embedding_input = EmbeddingInput::StringArray(string_records);
        let embedding_request = EmbeddingRequestBuilder::default()
            .model("text-embedding-ada-002".to_string())
            .input(embedding_input)
            .build()?;
        Ok(embedding_request)
    }

    async fn post_embedding_request<I: serde::ser::Serialize, O: serde::de::DeserializeOwned>(&self, request: I) -> Result<O, SemanticSearchError> {
        let path = "/embeddings";

        let request = reqwest::Client::new()
            .post(format!("{}{path}", self.client.api_base()))
            .bearer_auth(self.client.api_key())
            .headers(self.client.headers())
            .json(&request)
            .build()?;

        let reqwest_client = reqwest::Client::new();
        let response = reqwest_client.execute(request).await?;

        let status = response.status();
        let bytes = response.bytes().await?;

        if !status.is_success() {
            let wrapped_error: WrappedError =
                serde_json::from_slice(bytes.as_ref()).map_err(SemanticSearchError::JSONDeserialize)?;

            return Err(SemanticSearchError::ApiError(wrapped_error.error));
        }

        let response: O =
            serde_json::from_slice(bytes.as_ref()).map_err(SemanticSearchError::JSONDeserialize)?;
        Ok(response)
    }
}

#[derive(Debug, Clone)]
/// Client is a container for api key, base url, organization id, and backoff
/// configuration used to make API calls.
pub struct Client {
    api_key: String,
    api_base: String,
    org_id: String,
}

/// Default v1 API base url
pub const API_BASE: &str = "https://api.openai.com/v1";
/// Name for organization header
pub const ORGANIZATION_HEADER: &str = "OpenAI-Organization";

impl Client {
    pub fn api_base(&self) -> &str {
        &self.api_base
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    fn new(api_key: String) -> Self{
        Self { api_key: api_key, api_base: API_BASE.to_string(), org_id: Default::default() }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if !self.org_id.is_empty() {
            headers.insert(ORGANIZATION_HEADER, self.org_id.as_str().parse().unwrap());
        }
        headers
    }
}

/// Wrapper to deserialize the error object nested in "error" JSON key
#[derive(Debug, Deserialize)]
pub(crate) struct WrappedError {
    pub(crate) error: ApiError,
}

/// OpenAI API returns error object on failure
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub r#type: String,
    pub param: Option<serde_json::Value>,
    pub code: Option<serde_json::Value>,
}

#[derive(Debug)]
enum SemanticSearchError {
    ObsidianError(JsValue),
    WriteError(csv::Error),
    ConversionError(Box<dyn std::error::Error>),
    ReqwestError(reqwest::Error),
    JSONDeserialize(serde_json::Error),
    ApiError(ApiError),
    InvalidArgument(String),
}

impl std::fmt::Display for SemanticSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticSearchError::ObsidianError(e) => write!(f, "obsidian error; {}", e.as_string().unwrap()),
            SemanticSearchError::WriteError(e) => write!(f, "write error; {:?}", e.source()),
            SemanticSearchError::ConversionError(e) => write!(f, "conversion error; {:?}", e.source()),
            SemanticSearchError::ReqwestError(e) => write!(f, "reqwest error; {}", e),
            SemanticSearchError::JSONDeserialize(e) => write!(f, "JSONDeserialize error: {:?}", e),
            SemanticSearchError::ApiError(e) => write!(f, "API error: {}: {}", e.r#type, e.message),
            SemanticSearchError::InvalidArgument(e) => write!(f, "Invalid argument: {}", e),
        }
    }
}

impl From<csv::Error> for SemanticSearchError {
    fn from(value: csv::Error) -> Self {
        Self::WriteError(value)
    }
}

impl From<csv::IntoInnerError<Writer<Vec<u8>>>> for SemanticSearchError {
    fn from(value: csv::IntoInnerError<Writer<Vec<u8>>>) -> Self {
        Self::ConversionError(Box::new(value.into_error()))
    }
}

impl From<std::string::FromUtf8Error> for SemanticSearchError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::ConversionError(Box::new(value))
    }
}

impl From<wasm_bindgen::JsValue> for SemanticSearchError {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Self::ObsidianError(value)
    }
}

impl From<reqwest::Error> for SemanticSearchError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

impl From<EmbeddingRequestBuilderError> for SemanticSearchError {
    fn from(value: EmbeddingRequestBuilderError) -> Self {
        Self::InvalidArgument(value.to_string())
    }
}

impl std::error::Error for SemanticSearchError {
}

#[wasm_bindgen]
pub fn onload(plugin: &obsidian::Plugin) {
    console_log::init_with_level(log::Level::Debug).expect("");
    let cmd = ExampleCommand {
        id: JsString::from("example"),
        name: JsString::from("Example"),
        vault: plugin.app().vault(),
        client: Client::new(plugin.settings().apiKey())
    };
    debug!("ApiKey: {:?}", plugin.settings().apiKey());
    plugin.addCommand(JsValue::from(cmd))
}

const DATA_FILE_PATH: &str = "./input.csv";

struct DataRow<'a> {
    file_name: &'a str,
    header: &'a str,
    body: &'a str,
}


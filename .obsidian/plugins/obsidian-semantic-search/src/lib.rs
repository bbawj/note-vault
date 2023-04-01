mod obsidian;
mod embedding;
mod file_processor;
mod error;

use crate::embedding::EmbeddingRequestBuilderError;
use crate::embedding::EmbeddingRequestBuilder;
use std::error::Error;

use csv::{FromUtf8Error, Writer, Reader, ReaderBuilder, StringRecord};
use embedding::EmbeddingRequest;
use error::SemanticSearchError;
use error::WrappedError;
use file_processor::FileProcessor;
use js_sys::JsString;
use log::{debug, error};
use reqwest::header::HeaderMap;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use crate::embedding::EmbeddingInput;

const DATA_FILE_PATH: &str = "./input.csv";
const EMBEDDING_FILE_PATH: &str = "./embedding.csv";

#[wasm_bindgen]
pub struct PrepareCommand {
    id: JsString,
    name: JsString,
    file_processor: FileProcessor,
}

#[wasm_bindgen]
impl PrepareCommand {
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
        // obsidian::Notice::new(
        //     format!(
        //         "Number of markdown files: {}",
        //         self.vault.getMarkdownFiles().len()
        //     )
        //     .as_str(),
        // );
        let data = self.file_processor.generate_input().await.expect("failed to prepare input.csv");
        // TODO: only write if file does not exist/delete the file first
        match self.file_processor.write_to_path(data, DATA_FILE_PATH).await {
            Ok(()) => (),
            Err(e) => error!("{:?}", e),
        }
    }
}

#[wasm_bindgen]
pub struct GetEmbeddingsCommand {
    id: JsString,
    name: JsString,
    file_processor: FileProcessor,
    client: Client,
}

#[wasm_bindgen]
impl GetEmbeddingsCommand {
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
        match self.get_embeddings().await {
            Ok(()) => (),
            Err(e) => error!("{:?}", e),
        }
    }

    async fn get_embeddings(&self) -> Result<(), SemanticSearchError> {
        let input = self.file_processor.read_from_path(DATA_FILE_PATH).await?;
        let request = self.create_embedding_request(input.clone())?;
        let response = self.post_embedding_request(&request).await?;
        debug!("{:?}", response);

        // let filename_header = self.get_filename_header(input.clone())?;
        // let mut wtr = csv::Writer::from_writer(vec![]);
        // match request.input {
        //     EmbeddingInput::StringArray(arr) => {
        //         for (i, input) in arr.iter().enumerate() {
        //             let filename_header = match filename_header.get(i) {
        //                 None => return Err(SemanticSearchError::GetEmbeddingsError(format!("embedding for filename_header index {} is not found", i))),
        //                 Some(filename_header) => filename_header
        //             };
        //             let filename = &filename_header.0;
        //             let header = &filename_header.1;
        //             wtr.write_record(&[filename, header, &input])?;
        //         }
        //     }
        // }
        //
        // let data = String::from_utf8(wtr.into_inner()?)?;
        // let adapter = self.file_processor.adapter();
        // adapter.append(EMBEDDING_FILE_PATH.to_string(), data).await?;
        Ok(())
    }

    fn create_embedding_request(&self, input: String) -> Result<EmbeddingRequest, SemanticSearchError> {
        let string_records = self.get_content_to_embed(input)?;
        let embedding_input = EmbeddingInput::StringArray(string_records);
        let embedding_request = EmbeddingRequestBuilder::default()
            .model("text-embedding-ada-002".to_string())
            .input(embedding_input)
            .user(None)
            .build()?;
        Ok(embedding_request)
    }

    fn get_content_to_embed(&self, input: String) -> Result<Vec<String>, SemanticSearchError> {
        let mut reader = ReaderBuilder::new().trim(csv::Trim::All).flexible(false)
            .from_reader(input.as_bytes());
        let records = reader.records().collect::<Result<Vec<StringRecord>, csv::Error>>()?;
        let string_records = records.iter().map(|record| 
                           record.get(2).unwrap().to_string()
                          ).collect();
        Ok(string_records)
    }

    fn get_filename_header(&self, input: String) -> Result<Vec<(String, String)>, SemanticSearchError> {
        let mut reader = ReaderBuilder::new().trim(csv::Trim::All).flexible(false)
            .from_reader(input.as_bytes());
        let records = reader.records().collect::<Result<Vec<StringRecord>, csv::Error>>()?;
        let filename_header = records.iter().map(|record| 
                           (record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string())
                          ).collect();
        Ok(filename_header)
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
        Self { api_key, api_base: API_BASE.to_string(), org_id: Default::default() }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if !self.org_id.is_empty() {
            headers.insert(ORGANIZATION_HEADER, self.org_id.as_str().parse().unwrap());
        }
        headers
    }
}

#[wasm_bindgen]
pub fn onload(plugin: &obsidian::Plugin) {
    console_log::init_with_level(log::Level::Debug).expect("");
    let preparecmd = build_prepare_cmd(plugin);
    let getembeddingscmd = build_get_embeddings_cmd(plugin);
    debug!("ApiKey: {:?}", plugin.settings().apiKey());
    plugin.addCommand(JsValue::from(preparecmd));
    plugin.addCommand(JsValue::from(getembeddingscmd))
}

fn build_prepare_cmd(plugin: &obsidian::Plugin) -> PrepareCommand {
    let file_processor = FileProcessor::new(plugin.app().vault());
    PrepareCommand {
        id: JsString::from("prepare"),
        name: JsString::from("Prepare Command"),
        file_processor
    }
}

fn build_get_embeddings_cmd(plugin: &obsidian::Plugin) -> GetEmbeddingsCommand {
    let file_processor = FileProcessor::new(plugin.app().vault());
    GetEmbeddingsCommand {
        id: JsString::from("get"),
        name: JsString::from("Get Embeddings Command"),
        file_processor,
        client: Client::new(plugin.settings().apiKey())
    }
}


struct DataRow<'a> {
    file_name: &'a str,
    header: &'a str,
    body: &'a str,
}


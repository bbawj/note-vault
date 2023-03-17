mod obsidian;
mod embeddings;
use std::{
    io::{self, BufReader, BufRead},
    path::Path, collections::HashMap, fs::File,
};

use embeddings::{Client, CreateEmbeddingRequest};
use js_sys::JsString;
use log::debug;
use obsidian::Vault;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExampleCommand {
    id: JsString,
    name: JsString,
    vault: Vault,
    openai_client: Client,
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

    pub fn callback(&self) {
        obsidian::Notice::new(
            format!(
                "Number of markdown files: {}",
                self.vault.getMarkdownFiles().len()
            )
            .as_str(),
        );
    }
}

#[wasm_bindgen]
pub fn onload(plugin: &obsidian::Plugin) {
    console_log::init_with_level(log::Level::Debug).expect("");
    let cmd = ExampleCommand {
        id: JsString::from("example"),
        name: JsString::from("Example"),
        vault: plugin.app().vault(),
        openai_client: Client::new().with_api_key(plugin.settings().apiKey())
    };
    debug!("ApiKey: {:?}", plugin.settings().apiKey());
    plugin.addCommand(JsValue::from(cmd))
}

fn process_files(files: Vec<&obsidian::TFile>) -> std::io::Result<()> {
    let mut file_to_section: HashMap<String, HashMap<String, String>> = HashMap::new();
    for file in files {
        let path = file.path();
        let sections_to_content = extract_sections(&path)?;
        file_to_section.insert(file.name(), sections_to_content);
    }
    Ok(())
}

fn extract_sections(path: &str) -> std::io::Result<HashMap<String, String>> {
    let mut header_to_content: HashMap<String, String> = HashMap::new();
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut header = "".to_string();
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        if line.starts_with("##") {
            header = line.replace("#", "").to_string();
        } else {
            let old = header_to_content.get(&header);
            if old.is_some() {
                header_to_content.insert(header.to_string(), old.unwrap().to_owned() + &line + "\n");
            }
        }
    }
    Ok(header_to_content)
}

fn create_embedding_request() {
    let request = CreateEmbeddingRequestArgs::default();
}

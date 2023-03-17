use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "obsidian")]
extern "C" {
    pub type Plugin;
    pub type App;
    pub type Vault;
    pub type TFile;
    pub type semanticSearchSettings;

    #[wasm_bindgen(structural, method)]
    pub fn addCommand(this: &Plugin, command: JsValue);

    #[wasm_bindgen(method, getter)]
    pub fn app(this: &Plugin) -> App;
    #[wasm_bindgen(method, getter)]
    pub fn settings(this: &Plugin) -> semanticSearchSettings;
    #[wasm_bindgen(method, getter)]
    pub fn apiKey(this: &semanticSearchSettings) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn vault(this: &App) -> Vault;

    #[wasm_bindgen(method)]
    pub fn getMarkdownFiles(this: &Vault) -> Vec<TFile>;
    #[wasm_bindgen(method, getter)]
    pub fn path(this: &TFile) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &TFile) -> String;

    pub type Notice;

    #[wasm_bindgen(constructor)]
    pub fn new(message: &str) -> Notice;
}

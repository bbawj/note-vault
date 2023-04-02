use js_sys::JsString;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{HtmlElement, HtmlInputElement};

#[wasm_bindgen]
pub struct QueryModal {
    limit: JsString,
    emptyStateText: JsString,
    inputEl: HtmlInputElement,
    resultContainerEl: HtmlElement,
}

// #[wasm_bindgen]
// impl QueryModal {
//   // Returns all available suggestions.
//   pub fn getSuggestions(&self, query: String) -> Vec<String> {
//       vec!["ADa".to_string()]
//   }

  // Renders each suggestion item.
  // pub fn renderSuggestion(book: Book, el: HTMLElement) {
  //   el.createEl("div", { text: "allo" });
  //   el.createEl("small", { text: book.author });
  // }
  //
  // // Perform action on the selected suggestion.
  // onChooseSuggestion(book: Book, evt: MouseEvent | KeyboardEvent) {
  //   new Notice(`Selected ${book.title}`);
  // }
// }


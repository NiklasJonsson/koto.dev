use {js_sys::Function, wasm_bindgen::prelude::*};

#[wasm_bindgen]
extern "C" {
    pub type Ace;
    pub type AceEditor;
    pub type AceSession;

    #[wasm_bindgen(method)]
    pub fn edit(this: &Ace, id: &str) -> AceEditor;

    #[wasm_bindgen(method, js_name = getSession)]
    pub fn get_session(this: &AceEditor) -> AceSession;

    #[wasm_bindgen(method, js_name = setTheme)]
    pub fn set_theme(this: &AceEditor, theme: &str);

    #[wasm_bindgen(method, js_name = setShowPrintMargin)]
    pub fn set_show_print_margin(this: &AceEditor, value: bool);

    #[wasm_bindgen(method, js_name = setMode)]
    pub fn set_mode(this: &AceSession, mode: &str);

    #[wasm_bindgen(method, js_name = getValue)]
    pub fn get_value(this: &AceSession) -> String;

    #[wasm_bindgen(method)]
    pub fn on(this: &AceSession, event_name: &str, callback: &Function);
}

#[wasm_bindgen(inline_js = "export function get_ace() { return ace; }")]
extern "C" {
    pub fn get_ace() -> Ace;
}

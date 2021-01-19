use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_tree_sitter_sys::*;

#[wasm_bindgen_test]
async fn new() {
    async fn inner() -> Result<(), JsValue> {
        let _options = <ParseOptions as Default>::default();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn included_ranges() {
    async fn inner() -> Result<(), JsValue> {
        let options = <ParseOptions as Default>::default();
        let _included_ranges = options.included_ranges();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

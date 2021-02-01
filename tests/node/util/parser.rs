use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;
use web_tree_sitter_sys::*;

pub(crate) async fn init() -> Result<(), JsValue> {
    JsFuture::from(Parser::init()).await?;
    Ok(())
}

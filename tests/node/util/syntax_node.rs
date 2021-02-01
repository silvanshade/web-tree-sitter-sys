use wasm_bindgen::prelude::*;
use web_tree_sitter_sys::*;

pub(crate) async fn make() -> Result<Option<SyntaxNode>, JsValue> {
    let tree = crate::util::tree::make().await?;
    let node = tree.map(|tree| tree.root_node());
    Ok(node)
}

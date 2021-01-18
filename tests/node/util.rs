use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type Require;

    pub static require: Require;

    #[wasm_bindgen(method)]
    pub fn resolve(this: &Require, request: &str, options: Option<Object>) -> String;
}

pub(crate) mod parser {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::*;
    use web_tree_sitter_sys::*;

    pub(crate) async fn init() -> Result<(), JsValue> {
        JsFuture::from(Parser::init()).await?;
        Ok(())
    }
}

pub(crate) mod language {
    use std::{error::Error, path::PathBuf};
    use wasm_bindgen::{prelude::*, JsCast};
    use wasm_bindgen_futures::*;
    use web_tree_sitter_sys::*;

    pub(crate) fn url(language: &str) -> Result<String, Box<dyn Error>> {
        let mut path = PathBuf::new();
        path.push(env!("CARGO_MANIFEST_DIR"));
        path.push("assets");
        path.push(format!("tree-sitter-{}.wasm", language));
        let path = match path.into_os_string().into_string() {
            Ok(path) => Ok::<_, Box<dyn Error>>(path),
            Err(err) => Err(format!("failed to convert path to string `{:?}`", err).into()),
        }?;
        Ok(super::require.resolve(&path, None))
    }

    pub(crate) async fn load() -> Result<Language, JsValue> {
        let id = "javascript";
        let url = crate::util::language::url(id);
        let url = url.expect(&format!("failed to obtain url for language '{}'", id));
        let language = JsFuture::from(Language::load(&url)).await?;
        let language = language.unchecked_into::<Language>();
        Ok(language)
    }
}

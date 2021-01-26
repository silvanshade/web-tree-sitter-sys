use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn load_bytes() {
    async fn inner() -> Result<(), JsValue> {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_tree_sitter_sys::Language;
        let bytes: &[u8] = include_bytes!("../../assets/tree-sitter-javascript.wasm");
        let language = JsFuture::from(Language::load_bytes(&bytes.into())).await?;
        language.unchecked_into::<Language>();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn load_path() {
    async fn inner() -> Result<(), JsValue> {
        crate::util::parser::init().await?;
        crate::util::language::load().await?;
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn version() {
    async fn inner() -> Result<(), JsValue> {
        crate::util::parser::init().await?;
        let language = crate::util::language::load().await?;
        assert_eq!(11, language.version());
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn field_count() {
    async fn inner() -> Result<(), JsValue> {
        crate::util::parser::init().await?;
        let language = crate::util::language::load().await?;
        assert_eq!(34, language.field_count());
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn field_name_for_id() {
    async fn inner() -> Result<(), JsValue> {
        crate::util::parser::init().await?;
        let language = crate::util::language::load().await?;
        let name = language.field_id_for_name("alias");
        assert_eq!(Some(1), name);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn field_id_for_name() {
    async fn inner() -> Result<(), JsValue> {
        crate::util::parser::init().await?;
        let language = crate::util::language::load().await?;
        let id = language.field_name_for_id(1);
        assert_eq!(Some("alias".into()), id);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn query() {
    async fn inner() -> Result<(), JsValue> {
        crate::util::parser::init().await?;
        let language = crate::util::language::load().await?;
        let query = r###"
        (function_declaration name: (identifier) @fn-def)
        (call_expression function: (identifier) @fn-ref)
        "###
        .into();
        language.query(&query)?;
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn query_throws() {
    async fn inner() -> Result<(), JsValue> {
        crate::util::parser::init().await?;
        let language = crate::util::language::load().await?;
        let query = r###"
        (function_declaration wat)
        "###
        .into();
        let _query = language.query(&query)?;
        Ok(())
    }
    assert!(inner().await.is_err());
}

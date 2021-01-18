use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn load() {
    async fn inner() -> Result<(), JsValue> {
        crate::util::parser::init().await?;
        crate::util::language::load().await.unwrap();
        Ok(())
    }
    inner().await.unwrap();
}

#[wasm_bindgen_test]
async fn field_count() {
    async fn inner() -> Result<(), JsValue> {
        crate::util::parser::init().await?;
        let language = crate::util::language::load().await.unwrap();
        assert_eq!(34, language.field_count());
        Ok(())
    }
    inner().await.unwrap();
}

#[wasm_bindgen_test]
async fn version() {
    async fn inner() -> Result<(), JsValue> {
        crate::util::parser::init().await?;
        let language = crate::util::language::load().await.unwrap();
        assert_eq!(11, language.version());
        Ok(())
    }
    inner().await.unwrap();
}

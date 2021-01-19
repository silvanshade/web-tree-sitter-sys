use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn copy() {
    async fn inner() -> Result<(), JsValue> {
        let tree = crate::util::tree::make().await?.unwrap();
        let _tree = tree.copy();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn delete() {
    async fn inner() -> Result<(), JsValue> {
        let tree = crate::util::tree::make().await?.unwrap();
        let _tree = tree.delete();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn edit() {
    async fn inner() -> Result<(), JsValue> {
        let tree = crate::util::tree::make().await?.unwrap();
        let edit = Default::default();
        let _tree = tree.edit(&edit);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn walk() {
    async fn inner() -> Result<(), JsValue> {
        let tree = crate::util::tree::make().await?.unwrap();
        let _cursor = tree.walk();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn get_changed_ranges() {
    async fn inner() -> Result<(), JsValue> {
        let this = crate::util::tree::make().await?.unwrap();
        let that = this.copy();
        let _changed = this.get_changed_ranges(&that);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn get_language() {
    async fn inner() -> Result<(), JsValue> {
        let tree = crate::util::tree::make().await?.unwrap();
        let _language = tree.get_language();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

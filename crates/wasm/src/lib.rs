use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn init_wasm() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn from_cheese_mass(cheese_mass: f64) -> Result<JsValue, JsValue> {
    let items = cheesecalc::from_cheese_mass(cheese_mass);

    serde_wasm_bindgen::to_value(&items).map_err(|e| e.into())
}

#[wasm_bindgen]
pub fn from_pasta_mass(pasta_mass: f64) -> Result<JsValue, JsValue> {
    let items = cheesecalc::from_pasta_mass(pasta_mass);

    serde_wasm_bindgen::to_value(&items).map_err(|e| e.into())
}

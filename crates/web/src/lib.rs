pub mod todo;
mod update;
mod command;

use wasm_bindgen::prelude::*;
use shared::model::Model as ModelShared;

#[wasm_bindgen]
pub struct Model(ModelShared);

#[wasm_bindgen]
pub fn init() -> Model{
        Model(ModelShared::default())
}

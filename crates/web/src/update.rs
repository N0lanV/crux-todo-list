use wasm_bindgen::prelude::*;
use shared::{
        model::Model,
        message::Message,
        command::Command,
        update::update as update_shared
};
use crate::command::execute;

#[wasm_bindgen]
pub fn update(model: &mut crate::Model, message: JsValue){
        let message: Message = serde_wasm_bindgen::from_value(message)
                .expect("Serde failed to parse message");
        update_impl(&mut model.0, message);
}
fn update_impl(
        model: &mut Model,
        message: Message
) -> Option<Command>{
        let command = update_shared(model, message)?;
        let message = execute(command)?;
        update_impl(model, message)
}
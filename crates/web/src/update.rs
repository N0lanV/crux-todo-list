use wasm_bindgen::prelude::*;
use shared::{
        message::Message,
        update::update as update_shared
};
use crate::command::execute;

#[wasm_bindgen]
pub async fn update(model: &mut crate::Model, message: JsValue) -> Result<JsValue, JsValue>{
        let mut message: Option<Message> = Some(serde_wasm_bindgen::from_value(message)
                .expect("Serde failed to parse message"));

        while message.is_some() {
                let Some(command) = update_shared(
                        &mut model.0,
                        message.expect("message is some")
                ) else { break };
                message = execute(command).await;
        }
        Ok(serde_wasm_bindgen::to_value(&true)
                .expect("Serde failed to serialized true"))
}
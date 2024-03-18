use shared::{
        command::Command,
        message::Message,
        model::Model,
        update::update as update_shared
};
use crate::command::execute;

pub fn update(
        model: &mut Model,
        message: Message
) -> Option<Command>{
        let command = update_shared(model, message)?;
        let message = execute(command)?;
        update(model, message)
}
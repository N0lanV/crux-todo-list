use crate::command::Command;
use crate::message::Message;
use crate::model::Model;
use crate::todo::update_todo;

pub fn update(model: &mut Model, message: Message) -> Option<Command>{
        match message {
                Message::Todo(_) => update_todo(model, message)
        }
}
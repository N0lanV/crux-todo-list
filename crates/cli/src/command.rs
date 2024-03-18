use shared::{
        command::Command,
        message::Message,
};
use crate::todo::execute_todo;

pub fn execute(command: Command) -> Option<Message>{
        match command {
                Command::Todo(_) => {
                        execute_todo(command)
                },
        }
}
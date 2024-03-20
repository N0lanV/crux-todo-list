#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::todo::MessageTodo;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Message{
        Todo(MessageTodo)
}
use crate::todo::CommandTodo;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Command{
        Todo(CommandTodo)
}
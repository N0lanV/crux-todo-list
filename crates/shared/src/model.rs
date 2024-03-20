#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::todo::ModelTodo;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Model{
        pub todo: ModelTodo,
}
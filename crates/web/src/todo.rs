use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use shared::{
        command::Command,
        message::Message,
        todo::{
                CommandTodo,
                Priority,
                Task
        }
};

#[derive(Deserialize, Serialize)]
struct View{
        pub task_new_title: String,
        pub task_new_priority: Priority,
        pub task_list: Vec<Task>
}

#[wasm_bindgen]
pub fn render_todo(model: &crate::Model) -> JsValue{
        let todo = &model.0.todo;
        let view = View{
                task_new_title: todo.task_new_title.clone(),
                task_new_priority: todo.task_new_priority.clone(),
                task_list: model.0.todo.task_map.values()
                        .cloned()
                        .collect()
        };
        serde_wasm_bindgen::to_value(&view).expect("")
}

pub fn execute_todo(command: Command) -> Option<Message>{
        match command {
                Command::Todo(CommandTodo::Persist(_)) => None,
                Command::Todo(CommandTodo::Load) => None
        }
}
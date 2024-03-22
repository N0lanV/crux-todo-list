use rexie::{Index, ObjectStore, Rexie, TransactionMode};
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
use shared::todo::MessageTodo;

const DATABASE: &str = "database";
const STORE: &str = "todos";

#[derive(Deserialize, Serialize)]
struct View{
        pub task_new_id: Option<String>,
        pub task_new_title: String,
        pub task_new_priority: Priority,
        pub task_list: Vec<Task>
}

#[wasm_bindgen]
pub fn render_todo(model: &crate::Model) -> JsValue{
        let todo = &model.0.todo;
        let view = View{
                task_new_id: todo.task_new_id.map(|id| id.to_string()),
                task_new_title: todo.task_new_title.clone(),
                task_new_priority: todo.task_new_priority.clone(),
                task_list: model.0.todo.task_map.values()
                        .cloned()
                        .collect()
        };
        serde_wasm_bindgen::to_value(&view).expect("")
}

async fn build_database() -> Option<Rexie> {
        Rexie::builder(DATABASE)
                .version(1)
                .add_object_store(
                        ObjectStore::new(STORE)
                                .key_path("id")
                                .add_index(Index::new("id", "id").unique(true)),
                )
                .build()
                .await
                .ok()
}

pub async fn execute_todo(command: Command) -> Option<Message>{
        match command {
                Command::Todo(CommandTodo::Persist(task_list)) => {
                        let database = build_database().await?;
                        let transaction = database.transaction(
                                &[STORE],
                                TransactionMode::ReadWrite
                        ).ok()?;
                        let todos = transaction.store(STORE).ok()?;
                        
                        todos.clear().await.ok()?;
                        for task in task_list {
                                if let Ok(task) = serde_wasm_bindgen::to_value(&task){
                                        todos.add(&task, None).await.ok()?;
                                }
                        }
                        
                        transaction.done().await.ok()?;
                        None
                },
                Command::Todo(CommandTodo::Load) => {
                        let database = build_database().await?;
                        let transaction = database.transaction(
                                &[STORE],
                                TransactionMode::ReadOnly
                        ).ok()?;
                        let todos = transaction.store(STORE).ok()?;
                        
                        let task_list = todos.get_all(None, None, None, None)
                                .await
                                .ok()?;
                        let task_list = task_list
                                .into_iter()
                                .map(|(_, task)| serde_wasm_bindgen::from_value::<Task>(task).ok())
                                .collect::<Option<Vec<Task>>>()?;
                        
                        Some(Message::Todo(
                                MessageTodo::ReplaceTaskList(task_list)
                        ))
                }
        }
}
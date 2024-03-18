use std::collections::HashMap;
use uuid::Uuid;
use crate::{
        command::Command,
        model::Model,
        message::Message
};

#[derive(Debug, Clone, Default)]
pub struct ModelTodo {
        pub task_new_title: String,
        pub task_new_priority: Priority,
        pub task_map: HashMap<Uuid, Task>
}

#[derive(Debug, Clone, Default)]
pub struct Task{
        pub id: Uuid,
        pub title: String,
        pub priority:Priority
}

#[derive(Debug, Clone, Default)]
pub enum Priority{
        #[default]
        Low,
        Medium,
        High
}
impl From<Priority> for String{
        fn from(priority: Priority) -> Self {
                match priority {
                        Priority::Low => "low".to_string(),
                        Priority::Medium => "medium".to_string(),
                        Priority::High => "high".to_string()
                }
        }
}
pub enum CommandTodo{
        Persist(Vec<Task>),
        Load
}

pub enum MessageTodo{
        UpdateTaskNewTitle(String),
        UpdateTaskNewPriority(Priority),
        UpsertTask { id: Option<Uuid>, title: String, priority: Priority },
        RemoveTask(Uuid),
        LoadTaskList,
        ReplaceTaskList(Vec<Task>),
}

pub fn update_todo(
        model: &mut Model,
        message: Message
) -> Option<Command>{
        match message {
                Message::Todo(MessageTodo::UpdateTaskNewTitle(title))=> {
                        model.todo.task_new_title = title.clone();
                        None
                },
                Message::Todo(MessageTodo::UpdateTaskNewPriority(priority))=> {
                        model.todo.task_new_priority = priority.clone();
                        None
                },
                Message::Todo(
                        MessageTodo::UpsertTask {
                                id,
                                title,
                                priority
                        }
                )=> {
                        let id = id.unwrap_or(Uuid::new_v4());
                        model.todo.task_map.insert(
                                id,
                                Task{
                                        id,
                                        title: title.clone(),
                                        priority: priority.clone()
                                }
                        );
                        model.todo.task_new_title = "".to_string();
                        model.todo.task_new_priority = Priority::default();
                        Some(Command::Todo(CommandTodo::Persist(
                                model.todo.task_map.values().cloned().collect()
                        )))
                },
                Message::Todo(MessageTodo::RemoveTask(task_id))=> {
                        model.todo.task_map.remove(&task_id);
                        model.todo.task_new_title = "".to_string();
                        model.todo.task_new_priority = Priority::default();
                        Some(Command::Todo(CommandTodo::Persist(
                                model.todo.task_map.values().cloned().collect()
                        )))
                },
                Message::Todo(MessageTodo::LoadTaskList)=> {
                        Some(Command::Todo(CommandTodo::Load))
                },
                Message::Todo(MessageTodo::ReplaceTaskList(task_list))=> {
                        model.todo.task_map = task_list
                                .iter()
                                .map(|task| (task.id, task.clone()))
                                .collect();
                        None
                }
        }
}
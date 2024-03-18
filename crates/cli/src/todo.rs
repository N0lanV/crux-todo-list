use clap::ValueEnum;
use std::fs::File;
use std::io::{Read, Write};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use shared::{
        command::Command,
        message::Message,
        todo::{
                CommandTodo,
                Task,
                MessageTodo,
                Priority
        }
};

#[derive(Debug)]
pub enum ErrorTodo{
        UuidParsingFailed
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskCli{
        pub id: String,
        pub title: String,
        pub priority: PriorityCli
}

impl From<Task> for TaskCli{
        fn from(task: Task) -> Self {
                TaskCli{
                        id: task.id.to_string(),
                        title: task.title,
                        priority: task.priority.into(),
                }
        }
}
impl TryFrom<TaskCli> for Task{
        type Error = ErrorTodo;
        fn try_from(task_cli: TaskCli) -> Result<Self, Self::Error> {
                Ok(Task{
                        id: Uuid::parse_str(&task_cli.id)
                                .map_err(|_| ErrorTodo::UuidParsingFailed)?,
                        title: task_cli.title,
                        priority: task_cli.priority.into(),
                })
        }
}

#[derive(Deserialize, Serialize, ValueEnum, Clone, Debug)]
pub enum PriorityCli {
        Low,
        Medium,
        High
}
impl From<PriorityCli> for Priority{
        fn from(priority_cli: PriorityCli) -> Self {
                match priority_cli {
                        PriorityCli::Low => Priority::Low,
                        PriorityCli::Medium => Priority::Medium,
                        PriorityCli::High => Priority::High
                }
        }
}
impl From<Priority> for PriorityCli {
        fn from(priority: Priority) -> Self {
                match priority {
                        Priority::Low => PriorityCli::Low,
                        Priority::Medium => PriorityCli::Medium,
                        Priority::High => PriorityCli::High
                }
        }
}

pub fn execute_todo(command: Command) -> Option<Message>{
        match command {
                Command::Todo(CommandTodo::Load) => {
                        let mut file = File::open("database.bin").ok()?;
                        let mut content = Vec::<u8>::new();
                        file.read_to_end(&mut content).ok()?;
                        let task_list: Vec<TaskCli> = bincode::deserialize(&content).ok()?;
                        let Ok(task_list) = task_list.into_iter()
                                .map(|task| task.try_into())
                                .collect() else {
                                return None;
                        };
                        Some(Message::Todo(
                                MessageTodo::ReplaceTaskList(task_list)
                        ))
                },
                Command::Todo(CommandTodo::Persist(task_list)) => {
                        let mut file = File::create("database.bin").ok()?;
                        let task_list: Vec<TaskCli> = task_list.into_iter()
                                .map(|task| task.into())
                                .collect();
                        let _ = file.write_all(
                                &bincode::serialize(&task_list).ok()?
                        );
                        None
                }
        }
}
use std::fs::File;
use std::io::{Read, Write};
use shared::{
        command::Command,
        message::Message,
        todo::{
                CommandTodo,
                Task,
                MessageTodo,
        }
};

pub fn execute_todo(command: Command) -> Option<Message>{
        match command {
                Command::Todo(CommandTodo::Load) => {
                        let mut file = File::open("database.bin").ok()?;
                        let mut content = Vec::<u8>::new();
                        file.read_to_end(&mut content).ok()?;
                        let task_list: Vec<Task> = bincode::deserialize(&content).ok()?;
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
                        let _ = file.write_all(
                                &bincode::serialize(&task_list).ok()?
                        );
                        None
                }
        }
}
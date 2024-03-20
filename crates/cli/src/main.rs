mod command;
mod update;
pub mod todo;

use clap::{Parser, Subcommand};
use uuid::Uuid;
use shared::{
    message::Message,
    model::Model,
    todo::{MessageTodo, Priority}
};
use update::update;

/// Todo list
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands{
    /// Add/update task
    UpsertTask {
        #[arg(short, long)]
        id: Option<Uuid>,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        priority: Priority
    },
    /// Remove task
    RemoveTask {
        id: Uuid,
    },
}



fn main() {
    let args = Args::parse();
    let mut model = Model::default();
    update(
        &mut model,
        Message::Todo(MessageTodo::LoadTaskList)
    );

    match args.command {
        Commands::UpsertTask { id, title, priority  } => {
            update(
                &mut model,
                Message::Todo(MessageTodo::UpsertTask {
                    id,
                    title,
                    priority: priority.into()
                }));
            println!("{:?}", model);
        }
        Commands::RemoveTask { id } => {
            update(
                &mut model,
                Message::Todo(
                    MessageTodo::RemoveTask(id)
                )
            );
            println!("{:?}", model);
        }
    }
}
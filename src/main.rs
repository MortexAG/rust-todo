use clap::{Parser};
use std::{error::Error};
use colored::*;
mod task;
use task::*;
#[derive (Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg (short, long, help="add, del, done, edit", default_value="list")]
    action: String,

    #[arg (short, long, help = "task name", required_if_eq("action", "add"))]
    task: Option<String>,

    #[arg (short, long, help = "task id")]
    id: Option<u32>,
    #[arg (short, long, help = "new name")]
    name: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("===== SIMPLE TASK LIST =====\n");
    let args = Args::parse(); // use this later
    
    let mut task_list = load_tasks()?;

    match args.action.as_str() {
        "add" => {
            let task_name = args.task.clone().ok_or("Task name required for adding a task\n")?;
            let task = Task::new(task_name, false, task_list.next_id)?;
            add_task(task, &mut task_list)?;
            save_tasks(&task_list)?;
            list_tasks(&task_list)?;
        },
        "list" => {
            list_tasks(&task_list)?;
        },
        "del" => {
            // since args.id is option u32 we need to unwrap it first
            if let Some(id) = args.id {
                delete_task(id, &mut task_list)?;
                save_tasks(&task_list)?;
            } else {
                eprintln!("{}\n", format!("You must provide an ID to delete.").red().bold());
                println!("Usage: --action del --id <id>\n");
            }
            list_tasks(&task_list)?;
        },
        "done" => {
            // since args.id is option u32 we need to unwrap it first
            if let Some(id) = args.id {
                mark_done(id, &mut task_list)?;
                save_tasks(&task_list)?;
            } else {
                eprintln!("{}\n", format!("You must provide an ID to mark done.").red().bold());
                println!("Usage: --action done --id <id>\n");
            }
            list_tasks(&task_list)?;
        },
        "edit" => {
            // since args.id is option u32 we need to unwrap it first
            if let (Some(id), Some(name)) = (args.id, args.name.as_deref()) {
                edit_name(name, id, &mut task_list)?;
                save_tasks(&task_list)?;
            } else {
                eprintln!("{}\n", format!("You must provide an ID with -i or --id and New Name with -n or --name to edit the name.").red().bold());
                println!("Usage: --action edit --id <id> --name <new_name>\n");
            }
            list_tasks(&task_list)?;
        }

        _ => eprintln!("Unknown Action: {}\n", args.action),
    }
    println!("\n============================");
    Ok(())  
}


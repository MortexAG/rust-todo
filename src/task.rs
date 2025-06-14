// add a new task to the list
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{error::Error, fs};
use colored::*;

pub const PATH: &str = "./tasks.json";

/* correct some borrowing stuff in here now no function edit the file directly they edit the task_list in memory then the save function is run to edit the json
ths is supposed to be better for unit tests */
/* as far as i understand i am only working with a single instance of task_list coming from main.rs */

pub fn add_task(task: Task, task_list: &mut TaskList) -> Result<(), Box<dyn Error>> {
    //let mut task_list = load_tasks()?;
    task_list.tasks.push(task.clone());
    task_list.next_id +=1;
    //save_tasks(&task_list)?;
    Ok(())
}

// list all the tasks
pub fn list_tasks(task_list: &TaskList) -> Result<(), Box<dyn Error>>{
    println!("===== TASKS =====\n");

    for task in &task_list.tasks {
        let status = if task.done {
            "[x]".green().bold()
        } else {
            "[ ]".yellow()
        };

        let name = if task.done {
            task.name.green()
        } else {
            task.name.normal()
        };

    println!("- {} {} {}", status, name, format!("(ID: {})", task.id).blue());
}
        println!("\n=================");
    Ok(())
}

// delete a task by id
pub fn delete_task(task_id: u32, task_list: &mut TaskList) -> Result<(), Box<dyn Error>>{
    //let mut task_list = load_tasks()?;
    // we get the list length for checking later
    let original_len = task_list.tasks.len();
    // we extract only the task that has the same id as requested id
    task_list.tasks.retain(|task| task.id != task_id);
    if task_list.tasks.len() == original_len {
        println!("{}\n", format!("No Task With ID {} found.", task_id).red().bold());
        println!("Usage: --action del --id {}", task_id);
    }else {
        eprintln!("{}\n", format!("Deleted Task With ID {}.", task_id).green());
        //save_tasks(&task_list)?;
    }
    Ok(())
}

// mark the task as done
pub fn mark_done(task_id: u32, task_list: &mut TaskList) -> Result<(), Box<dyn Error>>{
    //let mut task_list = load_tasks()?;
    let mut found = false;

    for task in &mut task_list.tasks {
        if task.id == task_id {
            task.done = true;
            found = true;
            println!("{}\n", format!("Marked Task {} as Done", task_id).green().bold());
            break;
        }
    }
    if !found {
        println!("{}\n", format!("No Task With ID {} found.", task_id).red().bold());
        println!("Usage: --action done --id {}", task_id);
    }//else {
        //save_tasks(&task_list)?;
    //}

    Ok(())
}

// edit task name
pub fn edit_name(name: &str, task_id: u32, task_list: &mut TaskList) -> Result<(), Box<dyn Error>>{
    //let mut task_list = load_tasks()?;
    let mut found = false;
    let mut old_name: String = "Task".to_string();
    for task in &mut task_list.tasks{
        if task.id == task_id {
            old_name = task.name.clone();
            task.name = name.to_string();
            found = true;
        }
    }
    if !found {
        println!("{}\n", format!("No Task With ID {} found.", task_id).red().bold());
    }else {
        //save_tasks(&task_list)?;
        println!("{}\n", format!("Changed Task {} to {}", old_name, name).green().bold());
        println!("Usage: --action edit --id {} --name <new_name>", task_id);
        }
    Ok(())
}

// load tasks from the json file
pub fn load_tasks() -> Result<TaskList, Box<dyn Error>>{
    if let Ok(content) = fs::read_to_string(&PATH) {
        let task_list: TaskList = serde_json::from_str(&content)?;
        Ok(task_list)
    }else {
        Ok(TaskList { next_id: (1), tasks: Vec::new() })
    }
}

// save the tasks to the json file
pub fn save_tasks(task_list: &TaskList) -> Result<(), Box<dyn Error>>{
    let json = serde_json::to_string_pretty(task_list)?;
    fs::write(PATH, json)?;
    Ok(())
}

#[derive (Serialize, Deserialize)]
// the tasklist struct
pub struct TaskList{
    pub next_id: u32,
    pub tasks: Vec<Task>,
}
// serialize is used to handle json, clone is here to clone
#[derive (Serialize, Deserialize, Debug, Clone)]
// the task struct
pub struct Task {
    pub id: u32,
    pub uuid: Uuid,
    pub name: String,
    pub done: bool,
}

impl Task {
    pub fn new(name: String, done: bool, next_id: u32) -> Result<Task,  Box<dyn Error>>{ // useless error handling here but lets make it future proof
        let id: u32 = next_id;
        let uuid = Uuid::new_v4();
        Ok(Task {id,uuid, name, done})
    }
}

// testing, ALL TESTS WRITTEN BY AI

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_task(id: u32, name: &str, done: bool) -> Task {
        Task {
            id,
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            done,
        }
    }

    #[test]
    fn test_add_task() {
        let mut list = TaskList { next_id: 1, tasks: Vec::new() };
        let task = Task::new("Test Task".to_string(), false, list.next_id).unwrap();
        add_task(task, &mut list).unwrap();
        assert_eq!(list.tasks.len(), 1);
        assert_eq!(list.next_id, 2);
        assert_eq!(list.tasks[0].name, "Test Task");
    }

    #[test]
    fn test_delete_task_existing() {
        let mut list = TaskList { next_id: 2, tasks: vec![create_sample_task(1, "Test", false)] };
        let result = delete_task(1, &mut list);
        assert!(result.is_ok());
        assert_eq!(list.tasks.len(), 0);
    }

    #[test]
    fn test_delete_task_nonexistent() {
        let mut list = TaskList { next_id: 1, tasks: vec![create_sample_task(1, "Test", false)] };
        let result = delete_task(2, &mut list);
        assert!(result.is_ok());
        assert_eq!(list.tasks.len(), 1); // nothing deleted
    }

    #[test]
    fn test_mark_done_existing() {
        let mut list = TaskList { next_id: 2, tasks: vec![create_sample_task(1, "Test", false)] };
        let result = mark_done(1, &mut list);
        assert!(result.is_ok());
        assert!(list.tasks[0].done);
    }

    #[test]
    fn test_mark_done_nonexistent() {
        let mut list = TaskList { next_id: 1, tasks: vec![create_sample_task(1, "Test", false)] };
        let result = mark_done(2, &mut list);
        assert!(result.is_ok());
        assert!(!list.tasks[0].done); // unchanged
    }

    #[test]
    fn test_edit_name_existing() {
        let mut list = TaskList { next_id: 2, tasks: vec![create_sample_task(1, "Old Name", false)] };
        let result = edit_name("New Name", 1, &mut list);
        assert!(result.is_ok());
        assert_eq!(list.tasks[0].name, "New Name");
    }

    #[test]
    fn test_edit_name_nonexistent() {
        let mut list = TaskList { next_id: 1, tasks: vec![create_sample_task(1, "Old Name", false)] };
        let result = edit_name("New Name", 2, &mut list);
        assert!(result.is_ok());
        assert_eq!(list.tasks[0].name, "Old Name"); // unchanged
    }
}

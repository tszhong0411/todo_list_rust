use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, Write};

#[derive(Serialize, Deserialize)]
struct TodoItem {
    task: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(TodoItem { task, done: false });
    }

    fn mark_done(&mut self, index: usize) -> bool {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
            return true;
        }
        false
    }

    fn remove_task(&mut self, index: usize) -> bool {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            return true;
        }
        false
    }
}

const MAXIMUM_WIDTH: usize = 40;

fn print_instr_content(content: &str) {
    println!(
        "| {}{} |",
        content,
        " ".repeat(MAXIMUM_WIDTH - content.len() - 4)
    );
}

fn print_instructions() {
    println!("{}", "-".repeat(MAXIMUM_WIDTH));
    println!("|{}|", " ".repeat(MAXIMUM_WIDTH - 2));
    print_instr_content("1. Add a task");
    print_instr_content("2. List all tasks");
    print_instr_content("3. Mark a task as done");
    print_instr_content("4. Remove a task");
    print_instr_content("Type q to exit");
    println!("|{}|", " ".repeat(MAXIMUM_WIDTH - 2));
    println!("{}", "-".repeat(MAXIMUM_WIDTH));
}

fn get_tasks() -> io::Result<(File, TodoList)> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("tasks.json")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let todo_list: TodoList = if contents.is_empty() {
        TodoList::new()
    } else {
        serde_json::from_str(&contents).unwrap_or(TodoList::new())
    };

    Ok((file, todo_list))
}

fn add_task() {
    let (mut file, mut todo_list) = match get_tasks() {
        Ok(t) => t,
        Err(_) => panic!("Failed to get tasks"),
    };

    print!("Enter task description: ");
    io::stdout().flush().unwrap();
    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();

    todo_list.add_task(task.trim().to_string());

    file.set_len(0).unwrap();
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    file.write_all(serde_json::to_string(&todo_list).unwrap().as_bytes())
        .unwrap();
}

fn list_tasks() {
    let (_, todo_list) = match get_tasks() {
        Ok(t) => t,
        Err(_) => panic!("Failed to get tasks"),
    };

    if todo_list.tasks.is_empty() {
        println!("No tasks found!");
        return;
    }

    for (i, task) in todo_list.tasks.iter().enumerate() {
        println!(
            "{}. [{}] {}",
            i + 1,
            if task.done { "✓" } else { " " },
            task.task
        );
    }
}

fn mark_task_done() {
    let (mut file, mut todo_list) = match get_tasks() {
        Ok(t) => t,
        Err(_) => panic!("Failed to get tasks"),
    };

    list_tasks();

    print!("Enter task number to mark as done: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(task_num) = input.trim().parse::<usize>() {
        if todo_list.mark_done(task_num - 1) {
            file.set_len(0).unwrap();
            file.seek(std::io::SeekFrom::Start(0)).unwrap();
            file.write_all(serde_json::to_string(&todo_list).unwrap().as_bytes())
                .unwrap();
            println!("Task marked as done!");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Invalid input!");
    }
}

fn remove_task() {
    let (mut file, mut todo_list) = match get_tasks() {
        Ok(t) => t,
        Err(_) => panic!("Failed to get tasks"),
    };

    list_tasks();

    print!("Enter task number to remove: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(task_num) = input.trim().parse::<usize>() {
        if todo_list.remove_task(task_num - 1) {
            file.set_len(0).unwrap();
            file.seek(std::io::SeekFrom::Start(0)).unwrap();
            file.write_all(serde_json::to_string(&todo_list).unwrap().as_bytes())
                .unwrap();
            println!("Task removed successfully!");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Invalid input!");
    }
}

fn main() {
    println!("Welcome to use Todo!");
    print_instructions();

    loop {
        let mut answer = String::new();
        print!("Enter the number of the action: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut answer).unwrap();

        match answer.trim().to_lowercase().as_str() {
            "q" => break,
            "1" => add_task(),
            "2" => list_tasks(),
            "3" => mark_task_done(),
            "4" => remove_task(),
            _ => println!("Invalid option!"),
        }
    }
}

use std::io;

use todo::Todo;

use crate::todo::Task;

mod todo;

fn menu(option: &mut String) {
    println!("\nSelected an option: ");
    println!("1 - create a new task");
    println!("2 - delete a task");
    println!("3 - get a task by id");
    println!("4 - list tasks");
    println!("5 - exit\n");
    
    let result = io::stdin().read_line(option);

    match result {
        Err(_) => {
            println!("\nSomething went wrong!\n");
        },
        _ => ()
    }
}

fn create_task(todo: &mut Todo) {
    let mut task_name = String::new();

    let result = io::stdin().read_line(&mut task_name);

    match result {
        Ok(_) => {
            todo.add_task(Task::new(task_name.trim().to_string(), false));
        },
        Err(_) => {
            println!("\nSomething went wrong!\n");
        },
    }
}

fn remove_task(todo: &mut Todo) {
    let mut task_id = String::new();

    let result = io::stdin().read_line(&mut task_id);

    match result {
        Ok(_) => {
            let _ = todo.remove_task(task_id.trim().to_string());
        },
        Err(_) => {
            println!("\nSomething went wrong!\n");
        },
    }
}

fn get_task(todo: &Todo) {
    let mut task_id = String::new();

    let result = io::stdin().read_line(&mut task_id);

    match result {
        Ok(_) => {
            let result = todo.get_by_id(task_id.trim().to_string());

            if result.is_ok() {
                let task = result.unwrap();

                println!("\nTask with id: {} | id: {}, name: {}, done: {}", task.id, task.id, task.name, task.done);
            }
        },
        Err(_) => {
            println!("\nSomething went wrong!\n");
        },
    }
}

fn list_tasks(todo: &Todo) {
    println!();
    
    for task in todo.tasks.iter() {
        println!("Task -> {}, {}, {}", task.id, task.name, task.done);
    }

    println!();
}

fn main() {
    let mut todo = Todo::new();
    let mut input: String = String::new(); 

    loop {
        menu(&mut input);

        let option = input.trim();

        match option {
            "1" => {
                create_task(&mut todo);
            },
            "2" => {
                remove_task(&mut todo);
            },
            "3" => {
                get_task(&todo);
            },
            "4" => {
                list_tasks(&todo);
            },
            "5" => {
                println!("finishing program...");
                std::process::exit(0);
            },
            _ => {
                println!("\nInvalid option\n");
            }
        }

        input.clear();
    }
}

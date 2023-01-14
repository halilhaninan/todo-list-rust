use std::io;

struct Task {
    name: String,
    completed: bool,
}

fn main() {
    let mut tasks = vec![];

    loop {
        println!("What would you like to do?");
        println!("1. Add a task");
        println!("2. Mark a task as completed");
        println!("3. List all tasks");
        println!("4. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => {
                println!("Enter the task name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();
                tasks.push(Task {
                    name: name.to_string(),
                    completed: false,
                });
                println!("Task added!");
            }
            "2" => {
                println!("Enter the task number:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                let task_num = input.parse::<usize>().unwrap();
                if task_num < 1 || task_num > tasks.len() {
                    println!("Invalid task number!");
                } else {
                    tasks[task_num - 1].completed = true;
                    println!("Task marked as completed!");
                }
            }
            "3" => {
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}. {} {}", i + 1, task.name, if task.completed {"Completed"} else {"Not completed"} );
                }
            }
            "4" => {
                break;
            }
            _ => {
                println!("Invalid input!");
            }
        }
    }
}

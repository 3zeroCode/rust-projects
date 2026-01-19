use std::io::{self, Write};

struct TODO {
    id: u32,
    description: String,
}

fn main() {
    let mut todos: Vec<TODO> = Vec::new();
    let mut id_counter = 1;

    println!("-----Welcome to the TODO CLI APP-----");
    loop {
        println!("\n Commands are :[a]Add [l] list [u]update [r]remove [q] quit");
        io::stdout().flush().unwrap();

        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Erorr");
        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "a" | "add" => {
                println!("Add Task:");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                io::stdin()
                    .read_line(&mut desc)
                    .expect("error cant add task");

                todos.push(TODO {
                    id: id_counter,
                    description: desc.trim().to_string(),
                });

                println!("Task has been added {}", id_counter);
                id_counter += 1;
            }

            "l" | "list" => {
                if todos.is_empty() {
                    println!("The todo is empty please add task");
                } else {
                    for item in &todos {
                        println!("{}: {}", item.id, item.description);
                    }
                }
            }

            "u" | "update" => {
                println!("\nEnter the ID you want to update: ");
                io::stdout().flush().unwrap();
                let mut id_input: String = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("error! add the id again");
                let target_id: u32 = id_input.trim().parse().expect("Please type a number");

                print!("Enter new description: ");
                io::stdout().flush().unwrap();
                let mut new_desc = String::new();
                io::stdin().read_line(&mut new_desc).expect("Error");

                for item in &mut todos {
                    if item.id == target_id {
                        item.description = new_desc.trim().to_string();
                    }
                }
                println!("Update process complete.");
            }

            "r" | "remove" => {
                print!("Enter ID to remove: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Error");

                let target_id: u32 = id_input.trim().parse().expect("Please type a number");

                todos.retain(|t| t.id != target_id);
                println!("Task removed.");
            }
            "q" | "quit" => {
                break;
            }
            _ => {}
        }
    }
}

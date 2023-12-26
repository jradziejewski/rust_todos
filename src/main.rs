use std::io;

#[derive(Debug)]
struct Todo {
    idx: i32,
    text: String,
}


fn main() {
    let mut todos: Vec<Todo> = vec![];
    let mut idx: i32 = 1;

    loop {
        println!("Your todos: ");
        for todo in &todos {
            println!("{}: {}", todo.idx, todo.text);
        } 
        println!("\n --- \n");

        println!("Choose an option: ");
        println!("1 - Add a Todo");
        println!("2 - Remove a Todo");
        println!("3 - Edit a Todo");
        println!("0 - Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number");
                print!("\x1B[2J");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter the new Todo: ");
                let mut new_todo = String::new();
                io::stdin().read_line(&mut new_todo).expect("Failed to read line");
                add_todo(&mut todos, new_todo.trim().to_string(), &mut idx);
            }
            2 => {
                println!("Enter the index to remove: ");
                let mut idx_str = String::new();
                io::stdin().read_line(&mut idx_str).expect("Failed to read line");
                let index: i32 = match idx_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number");
                        print!("\x1B[2J");
                        continue;
                    }
                };
                remove_todo(&mut todos, index);
            }
            3 => {
                println!("Enter the todo index: ");
                let mut idx_str = String::new();
                io::stdin().read_line(&mut idx_str).expect("Failed to read line");
                let index: i32 = match idx_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number");
                        print!("\x1B[2J");
                        continue;
                    }
                };
                println!("Enter the new todo name: ");
                let mut new_todo = String::new();
                io::stdin().read_line(&mut new_todo).expect("Failed to read line");
                edit_todo(&mut todos, new_todo.trim().to_string(), index);
            }
            0 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Try again!");
            }
        }

        print!("\x1B[2J");
    }
}

fn add_todo(todos: &mut Vec<Todo>, new_todo: String, idx: &mut i32) {
    todos.push(Todo {
        idx: *idx,
        text: new_todo
    });

    *idx += 1;
}

fn remove_todo(todos: &mut Vec<Todo>, idx: i32) {
    if let Some(index) = todos.iter().position(|todo| todo.idx == idx) {
        todos.remove(index);
    } else {
        println!("Todo with idx {} not found", idx);
    }
}

fn edit_todo(todos: &mut Vec<Todo>, new_text: String, idx: i32) {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.idx == idx) {
        todo.text = new_text;
    } else {
        println!("Todo with idx {} not found", idx);
    }
}

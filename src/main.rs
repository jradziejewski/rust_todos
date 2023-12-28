mod todos;
use std::io;

use crate::todos::todo::Todo;
use crate::todos::todoList::TodoList;

fn main() {
    let mut todos = TodoList { entities: vec![] };
    let mut idx: usize = 1;

    loop {
        println!("Your todos: ");
        for todo in &todos {
            println!("{}", todo);
        } 
        println!("\n --- \n");

        println!("Choose an option: ");
        println!("1 - Add a Todo");
        println!("2 - Remove a Todo");
        println!("3 - Edit a Todo");
        println!("0 - Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: usize = match choice.trim().parse() {
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
                let new_todo = get_string();
                add_todo(&mut todos, new_todo.trim().to_string(), &mut idx);
            }
            2 => {
                println!("Enter the index to remove: ");
                let index: usize = get_usize().unwrap();
                remove_todo(&mut todos, index);
            }
            3 => {
                println!("Enter the todo index: ");
                let index = get_usize().unwrap();
                println!("Enter the new todo name: ");
                let new_todo = get_string(); 
                edit_todo(&mut todos, new_todo, index);
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

fn add_todo(todos: &mut TodoList, new_todo: String, idx: &mut usize) {
    todos.entities.push(Todo {
        idx: *idx,
        text: new_todo,
        completed: false
    });

    *idx += 1;
}

fn remove_todo(todos: &mut TodoList, idx: usize) {
    if let Some(index) = todos.entities.iter().position(|todo| todo.idx == idx) {
        todos.entities.remove(index);
    } else {
        println!("Todo with idx {} not found", idx);
    }
}

fn edit_todo(todos: &mut TodoList, new_text: String, idx: usize) {
    if let Some(todo) = todos.entities.iter_mut().find(|todo| todo.idx == idx) {
        todo.edit_todo(new_text);
    }
}

fn get_usize() -> Result<usize, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid input. Please enter a number".to_string()),
    }
}

fn get_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

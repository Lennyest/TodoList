use dialoguer::Select;
use std::fs;

fn main() {
    let mut list = vec![String::from("")];

    // Load the list from a file
    let list_string = fs::read_to_string("todo_list.txt");
    if list_string.is_ok() {
        list = list_string.unwrap().lines().map(|s| s.to_owned()).collect();
    }

    println!("Welcome to the Rust To-Do List App!\nPlease select an option below:");

    let selections = &["Add to-do", "Remove to-do", "Edit to-do", "View", "Exit"];
    loop {
        let selection = Select::new()
            .items(&selections[..])
            .default(0)
            .interact()
            .unwrap();

        match selections[selection] {
            "Exit" => {
                println!("Goodbye!");
                break;
            },
            "View" => {
                println!("Current list: {:?}", list);
            },
            "Add to-do" => {
                let new_item = dialoguer::Input::<String>::new()
                    .with_prompt("Enter a new to-do item")
                    .interact_text()
                    .unwrap();

                list.push(new_item.to_string());
            },
            "Remove to-do" => {
                let selection = Select::new()
                    .items(&list[..])
                    .default(0)
                    .interact()
                    .unwrap();

                if selection < list.len() {
                    println!("Removing item: {}", list[selection]);
                    list.remove(selection);
                } else {
                    println!("Invalid selection");
                }
            },
            "Edit to-do" => {
                println!("Which item do you want to edit?");
                let selection = Select::new()
                    .items(&list[..])
                    .default(0)
                    .interact()
                    .unwrap();

                if selection < list.len() {
                    println!("Editing item: {}", list[selection]);
                    let new_item = dialoguer::Input::<String>::new()
                        .with_prompt("Enter a new to-do item")
                        .interact_text()
                        .unwrap();

                    if new_item.to_string() == " " {
                        println!("Returning...");
                        continue;
                    }

                    list[selection] = new_item.to_string();
                } else {
                    println!("Invalid selection");
                }
            },
            _ => {},
        }

        // Save the list to a file
        let list_string = list.join("\n");
        fs::write("todo_list.txt", &list_string).unwrap();

        // Read the list from the file
        let list_string = fs::read_to_string("todo_list.txt").unwrap();
        list = list_string.lines().map(|s| s.to_owned()).collect();

        println!("Current list: {:?}", list);
    }
}
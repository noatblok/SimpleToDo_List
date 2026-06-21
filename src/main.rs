use inquire::{Text, Select};

const MENU_CHOICES: [&str; 4] = ["Add Item", "Remove Item", "List Items", "Exit"];
fn main() {
    let mut items: Vec<String> = vec![];
    loop {
        let ans = Select::new("What would you like to do?", MENU_CHOICES.to_vec())
            .prompt();
        match ans {
            Ok(choice) => {
                match choice {
                    "Add Item" => add_item(&mut items),
                    "Remove Item" => remove_item(&mut items),
                    "List Items" => list_items(&items),
                    "Exit" => break,
                    _ => println!("An error occured, try again.")
                }
            },
            Err(_) => println!("An error occured, try again.")
        }
    }
}

fn add_item(items: &mut Vec<String>) {
    clearscreen::clear().expect("Something went wrong clearing the terminal");
    let ans = Text::new("What would you like to add?")
        .prompt();
    match ans {
        Ok(answer) => {
            items.push(answer)
        },
        Err(_) => println!("An error occured, try again.")
    }
}

fn remove_item(items: &mut Vec<String>) {
    clearscreen::clear().expect("Something went wrong clearing the terminal");
    let choice = Select::new("What would you like to remove?", items.clone()).prompt();
    match choice {
        Ok(selected_item) => {
            let mut removal_index = usize::MAX;
            for i in 0..items.iter().len() {
                if items[i] == selected_item {
                    removal_index = i;
                }
            }
            if removal_index != usize::MAX {
                items.remove(removal_index);
            } else {
                println!("You fucked up... Somehow")
            }
        },
        Err(_) => println!("An error occured, try again.")
    }
}

fn list_items(items: &Vec<String>) {
    clearscreen::clear().expect("Something went wrong clearing the terminal");
    let out_text = items.join(", ");
    println!("Your list contains: \n {}", out_text);
}
use inquire::{Text, Select};

const MENU_CHOICES: [&str; 4] = ["Add Item", "Remove Item", "List Items", "Exit"];
struct ToDoItem {
    task: String,
    completed: bool
}
fn main() {
    let mut items: Vec<ToDoItem> = vec![];
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

fn add_item(items: &mut Vec<ToDoItem>) {
    clearscreen::clear().expect("Something went wrong clearing the terminal");
    let ans = Text::new("What would you like to add?")
        .prompt();
    match ans {
        Ok(answer) => {
            items.push(ToDoItem{task: answer, completed: false})
        },
        Err(_) => println!("An error occured, try again.")
    }
}

fn remove_item(items: &mut Vec<ToDoItem>) {
    clearscreen::clear().expect("Something went wrong clearing the terminal");
    let mut itemlist: Vec<&str> = vec![];
    for item in items.iter() {
        itemlist.push(&item.task)
    }

    let choice = Select::new("What would you like to remove?", itemlist).prompt();
    match choice {
        Ok(selected_item) => {
            if let Some(removal_index) = items.iter().position(|item| item.task == selected_item) {
                items.remove(removal_index);
            } else {
                println!("You fucked up... Somehow")
            }
        },
        Err(_) => println!("An error occured, try again.")
    }
}

fn list_items(items: &Vec<ToDoItem>) {
    clearscreen::clear().expect("Something went wrong clearing the terminal");
    let mut out_text = String::new();
    let mut i = 0;
    for item in items {
        let mut toadd = "";
        if i != items.iter().len() - 1 {
            toadd = "\n"
        }
        out_text.push_str(&format!("{}{}", &item.task, toadd));
        i += 1
    }

    println!("Your list contains:\n{}", out_text);
}
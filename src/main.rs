use inquire::Select;
use std::fs;
mod experience;

use experience::show_experience;

fn main() {
    println!("Hey there! I'm Karan Pargal, a full stack web3 developer and currently learning new technologies.");

    let options = vec!["About","Experience","Projects","Contact"];

    let choice = Select::new("What would you like to know?", options.clone()).prompt();

    match choice {
        Ok(choice) => {
            if choice == options[0] {
                println!("I'm a full stack web3 developer and currently learning new technologies.");
            }
            else if choice == options[1] {
                let file_path = "./experience/experience.json".to_owned();
                let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                let res = show_experience(&contents);
                match res {
                    Ok(_res) => println!(""),
                    Err(_) => println!("Error in experience.rs"),
                }
            }
            else if choice == options[2] {
                println!("These are my projects:");
            }
            else if choice == options[3] {
                println!("You can contact me at:");
            }
        },
        Err(_) => println!("You did not select a valid option"),
    }
}

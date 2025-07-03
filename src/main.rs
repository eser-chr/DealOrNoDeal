mod game;
use game::game::run_game;
use inquire::Select;

use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("/home/chris/RUSTY/deal/resources/ascii_art/welcome.txt").unwrap();
    let mut buf = String::new();
    // file.read(&buf);
    file.read_to_string(&mut buf).unwrap();
    println!("{}", buf);


    let options = vec![
        "Start Game",
        "Read Rules",
        "About",
        "Exit",
    ];

    let choice = Select::new("Choose an option:", options)
        .prompt();

    match choice {
        Ok(answer) => println!("You selected: {}", answer),
        Err(_) => println!("There was an error."),
    }


    // run_game();
}

use std::io;

fn main() {
    println!("Please Select an Option");
    println!("1. Play\n2. Watch AI\n3. Quit");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let choice: u32 = input.trim().parse::<u32>().unwrap();

    match choice {
        1 => ai_player(false),
        2 => ai_player(true),
        3 => println!("Hey, you can't quit yet!"),
        _ => println!("That's not an option!")
    }
}

fn game(ai_player) {

    if ai_player == true {
        
    }
}
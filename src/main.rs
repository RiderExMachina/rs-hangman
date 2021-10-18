use std::io;
use rand::Rng;

fn main() {
    println!("Please Select an Option");
    println!("1. Play\n2. Watch AI\n3. Quit");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let choice: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    match choice {
        1 => game(false),
        2 => game(true),
        3 => println!("Okay, goodbye!"),
        _ => {
            println!("That's not an option!");
            main()
        }
    }
}

fn game(ai_player: bool) {
    //if wordlist == false {
    let words = ["nah", "neatnesses", "bake", "mackerel"];
    //} 
    
    // Define "globals"
    let guessed = false;
    let max_bad_guesses = 6;
    let mut guesses = 0;
    let mut bad_guesses = 0;

    let incorrect_letters = [" "];
    let correct_letters = [" "];

    // TODO: change the generator range to be from 0 to len of words
    let word_num = rand::thread_rng().gen_range(0..words.len());
    //let word_num = 1;
    let word2guess = words[word_num];
    while guessed == false {
        if bad_guesses >= max_bad_guesses {
            println!("You were unable to guess the word \"{}\" in {} guesses!", word2guess, max_bad_guesses);
            break;
        }
       println!("{}", "_ ".repeat(word2guess.len()));
       println!("Incorrect Letters: {}", incorrect_letters.join(" "));
       println!("Correct Letters: {}", correct_letters.join(" "));

       let mut guess = String::new();
       io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        guess.pop();

       if guess == word2guess {
           guesses+=1;
           println!("You guessed the word {} in {} guesses!", word2guess, guesses);
           break;
       };
       if guess.len() == 1 {
           if word2guess.contains(&guess) {
                println!("Yes! {} is in the word!", guess);
        }
        else {
            println!("No, {} is not in the word.", guess);
            bad_guesses += 1;
        }
        guesses += 1;
    };
    //if ai_player == true {
    //}   
    }
}
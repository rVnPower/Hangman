use std::io;

fn display_hangman(lives: u8) {
    println!("
     ------------
    /           |
    |           {0}
    |          {2}{1}{3}
    |          {4} {5}
    |
    |
   / \\
    ",
    if lives < 6 {"0"} else {""},
    if lives < 5 {"|"} else {""},
    if lives < 4 {"/"} else {" "},
    if lives < 3 {"\\"} else {""},
    if lives < 2 {"/"} else {""},
    if lives < 1 {"\\"} else {""},
    )
}

fn mask(text: &str) -> String {
    let mut new = String::new();
    for c in text.chars() {
        if c.is_alphabetic() {
            new.push('_');
        } else {
            new.push(' ');
        }
    }

    new
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get input");
    input
}

fn verify_user_input(input: &String) -> bool {
    match input.chars().next() {
        Some(c) => {
            if c.is_ascii_alphabetic() {
                return true;
            } else {
                return false;
            }
        },
        None => return false,
    }
}

fn process_input(lives: &mut u8, hidden_word: &str, input: String) -> String {
    let to_check_char = input.chars().next().unwrap().to_lowercase().next();
    let mut new_masked_word = String::new();
    let mut hit = false;

    for c in hidden_word.chars() {
        let c = c.to_lowercase().next();
        if c == to_check_char {
            hit = true;
            new_masked_word.push(c.unwrap());
        } else {
            new_masked_word.push('_');
        }
    }

    if !hit {
        *lives -= 1;
    }

    new_masked_word
}

fn post_process(masked: String, processed: String) -> String {
    let mut new_masked_string = String::new();
    for (a, b) in masked.chars().zip(processed.chars()) {
        if a == ' ' || a != '_' {
            new_masked_string.push(a);
            continue;
        }
        else if a == '_' && b != '_' && b != ' ' {
            new_masked_string.push(b);
        } 
        else {
            new_masked_string.push('_');
        }
    }
    new_masked_string
}

fn check_if_won(masked: &String) -> bool {
    for c in masked.chars() {
        if c == '_' {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut lives: u8 = 6;
    let hidden_word = "hello world";
    let guessed_letters: Vec<String> = Vec::new();
    let mut masked_word = mask(&hidden_word);
    let mut processed_string = String::new();
    let mut additional_message = "";

    loop {
        clear();
        println!("{}", additional_message);
        display_hangman(lives);
        println!("\n{}", &masked_word);
        
        // Break when the whole word was all discovered
        if check_if_won(&masked_word) {
            println!("You win!");
            break;
        }
        // Lose
        if lives < 1 {
            println!("You lose!");
            break;
        }

        let input = get_user_input();
        if verify_user_input(&input) {
            processed_string = process_input(&mut lives, &hidden_word, input);
            masked_word = post_process(masked_word, processed_string);

        } else {
            additional_message = "Your input was not valid. Only alphabetic characters are accepted.";
            continue;
        }
    }
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

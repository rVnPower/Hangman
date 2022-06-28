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
    if lives < 4 {"/"} else {""},
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

fn main() {
    let lives = 6;
    let hidden_word = "Amogus";
    let guessed_letters: Vec<String> = Vec::new();
    let masked_word = mask(hidden_word);

    loop {
        clear();
        display_hangman(lives);
        break;
    }
    
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

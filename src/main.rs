use rand::Rng;
use std::io::{self, Write};
use std::collections::HashSet;

const WORDS: &str = include_str!("wordList.txt");

fn get_tries_for_difficulty(difficulty: u8) -> u8 {
    match difficulty {
        1 => 12,  // Easy mode gets 2 tries per limb
        2 => 9,  // Medium mode gets more tries
        3 => 6,  // Hard mode gets standard tries, 1 per limb
        _ => panic!("Difficulty must be 1, 2, or 3"),
    }
}

fn select_word_by_difficulty<'a>(word_list: &'a Vec<&str>, difficulty: u8) -> &'a str {
    let mut rng = rand::thread_rng();
    loop {
        let random_word = word_list[rng.gen_range(0..word_list.len())];
        let word_length = random_word.len();
        let matches_difficulty = match difficulty {
            1 => word_length <= 5,
            2 => word_length > 5 && word_length <= 8,
            3 => word_length > 8,
            _ => panic!("Difficulty must be 1, 2, or 3"),
        };
        if matches_difficulty {
            return random_word;
        }
    }
}

fn display_hangman(tries_left: u8, max_tries: u8) {
    let remaining_percentage = tries_left as f32 / max_tries as f32;
    let stages = [
        // 0% tries left
        "
          +---+
          |   |
          O   |
         /|\\  |
         / \\  |
              |
        =========",
        // ~16% tries left
        "
          +---+
          |   |
          O   |
         /|\\  |
         /    |
              |
        =========",
        // ~33% tries left
        "
          +---+
          |   |
          O   |
         /|\\  |
              |
              |
        =========",
        // ~50% tries left
        "
          +---+
          |   |
          O   |
         /|   |
              |
              |
        =========",
        // ~66% tries left
        "
          +---+
          |   |
          O   |
          |   |
              |
              |
        =========",
        // ~83% tries left
        "
          +---+
          |   |
          O   |
              |
              |
              |
        =========",
        // 100% tries left
        "
          +---+
          |   |
              |
              |
              |
              |
        ========="
    ];
    
    // Calculate stage based on percentage of tries remaining
    let stage_index = (remaining_percentage * 6.0).round() as usize;
    let stage_index = stage_index.min(6); // Ensure we don't go out of bounds
    
    println!("{}", stages[stage_index]);
}

fn display_word(word: &str, guessed_letters: &HashSet<char>) -> String {
    word.chars()
        .map(|c| if guessed_letters.contains(&c) { c } else { '_' })
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn play_game(word: &str, difficulty: u8) {
    let mut guessed_letters = HashSet::new();
    let max_tries = get_tries_for_difficulty(difficulty);
    let mut tries_left = max_tries;
    let word_chars: HashSet<char> = word.chars().collect();

    println!("\nWelcome to Hangman!");
    println!("The word has {} letters.", word.len());
    println!("You have {} guesses for this difficulty level.", max_tries);

    while tries_left > 0 {
        println!("\nTries left: {}", tries_left);
        display_hangman(tries_left, max_tries);
        println!("\nWord: {}", display_word(word, &guessed_letters));
        println!("Guessed letters: {}", guessed_letters.iter().collect::<Vec<&char>>().iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "));
        
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().to_lowercase();
        
        if guess.len() != 1 {
            println!("Please enter a single letter!");
            continue;
        }

        let guess_char = guess.chars().next().unwrap();
        
        if !guess_char.is_alphabetic() {
            println!("Please enter a letter!");
            continue;
        }

        if guessed_letters.contains(&guess_char) {
            println!("You already guessed that letter!");
            continue;
        }

        guessed_letters.insert(guess_char);

        if !word_chars.contains(&guess_char) {
            tries_left -= 1;
            println!("Wrong guess!");
        } else {
            println!("Good guess!");
        }

        // Check if won
        if word_chars.iter().all(|c| guessed_letters.contains(c)) {
            println!("\nCongratulations! You won!");
            println!("The word was: {}", word);
            return;
        }
    }

    // Game over
    display_hangman(0, max_tries);
    println!("\nGame Over! You ran out of tries.");
    println!("The word was: {}", word);
}

fn main() {
    let word_list: Vec<&str> = WORDS.lines().collect();
    
    loop {
        println!("\nSelect difficulty:");
        println!("1: Easy (5 letters or less, 12 guesses)");
        println!("2: Medium (6-8 letters, 9 guesses)");
        println!("3: Hard (9+ letters, 6 guesses)");
        println!("0: Quit");
        
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        
        match choice.trim().parse() {
            Ok(0) => {
                println!("Thanks for playing!");
                break;
            }
            Ok(difficulty @ 1..=3) => {
                let word = select_word_by_difficulty(&word_list, difficulty);
                play_game(word, difficulty);
            }
            _ => {
                println!("Invalid choice! Please enter 0, 1, 2, or 3.");
                continue;
            }
        }
        
        print!("\nPlay again? (y/n): ");
        io::stdout().flush().unwrap();
        
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");
        
        if play_again.trim().to_lowercase() != "y" {
            println!("Thanks for playing!");
            break;
        }
    }
}
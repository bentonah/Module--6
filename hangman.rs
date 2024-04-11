use std::collections::HashSet;
use std::io;
use std::io::Write;

// Define a struct to represent the Hangman game
struct Hangman {
    secret_word: String,            // The secret word to guess
    guessed_letters: HashSet<char>, // Set of guessed letters
    max_attempts: u8,               // Maximum number of attempts allowed
    attempts_used: u8,              // Number of attempts used
}

// Implementation block for Hangman struct
impl Hangman {
    // Constructor function to create a new instance of Hangman
    fn new(word: &str, max_attempts: u8) -> Hangman {
        Hangman {
            secret_word: word.to_lowercase(),
            guessed_letters: HashSet::new(),
            max_attempts,
            attempts_used: 0,
        }
    }

    // Function to display the current state of the word with guessed letters
    fn display_word(&self) {
        for ch in self.secret_word.chars() {
            if self.guessed_letters.contains(&ch) {
                print!("{} ", ch); // Print the letter if guessed
            } else {
                print!("_ "); // Print underscore if not guessed
            }
        }
        println!();
    }

    // Function to display the current status of the game
    fn display_status(&self) {
        println!("Current word:");
        self.display_word(); // Display the word with guessed letters
        println!("Attempts used: {}/{}", self.attempts_used, self.max_attempts);
    }

    // Function to check if a guessed letter is correct
    fn check_guess(&mut self, guess: char) -> bool {
        self.guessed_letters.insert(guess); // Add the guessed letter to the set
        if !self.secret_word.contains(guess) {
            self.attempts_used += 1; // Increment attempts if guess is incorrect
            return false;
        }
        true
    }

    // Function to check if the game is over (either by max attempts or word guessed)
    fn is_game_over(&self) -> bool {
        self.attempts_used >= self.max_attempts || self.is_word_guessed()
    }

    // Function to check if the word has been completely guessed
    fn is_word_guessed(&self) -> bool {
        self.secret_word.chars().all(|ch| self.guessed_letters.contains(&ch))
    }
}

// Main function to run the Hangman game
fn main() {
    println!("Welcome to Hangman!");

    let secret_word = "rust"; // Change this to set a different secret word
    let max_attempts = 6;

    let mut game = Hangman::new(secret_word, max_attempts); // Create a new game instance

    // Main game loop
    while !game.is_game_over() {
        println!("Guess a letter:");
        print!("> ");
        io::stdout().flush().unwrap(); // Flush the output buffer

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Convert input to lowercase char
        let guess = match guess.trim().chars().next() {
            Some(ch) => ch.to_lowercase().next().unwrap(),
            None => {
                println!("Please enter a valid letter.");
                continue;
            }
        };

        // Check if the guessed letter is correct and display feedback
        if game.check_guess(guess) {
            println!("Correct guess!");
        } else {
            println!("Incorrect guess!");
        }

        // Display the current status of the game
        game.display_status();
    }

    // Display final result of the game
    if game.is_word_guessed() {
        println!("Congratulations! You guessed the word: {}", game.secret_word);
    } else {
        println!("Game over! The word was: {}", game.secret_word);
    }
}

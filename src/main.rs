use rand::Rng;
use std::{cmp::Ordering, io, num::ParseIntError};

/// A struct representing a guess made by the user.
#[derive(Debug)]
pub struct Guess {
    value: u32,
}

impl Guess {
    /// Creates a new guess with the given value.
    /// 
    /// # Arguments
    /// * `value` - A `u32` representing the user's guess.
    /// 
    /// # Returns
    /// Returns a `Result`:
    /// - `Ok(Guess)` if the guess is within the valid range (1 to 100).
    /// - `Err(GuessError::InvalidRange)` if the guess is outside the valid range.
    ///
    /// # Example
    /// ```
    /// let guess = Guess::new(50);
    /// assert!(guess.is_ok());
    /// ```
    pub fn new(value: u32) -> Result<Guess, GuessError> {
        if value < 1 || value > 100 {
            return Err(GuessError::InvalidRange);
        }
        Ok(Guess { value })
    }

    /// Returns the value of the guess.
    /// 
    /// # Returns
    /// Returns the `u32` value of the guess.
    pub fn value(&self) -> u32 {
        self.value
    }
}

/// A struct representing the number of guesses made by the user.
#[derive(Debug)]
pub struct GuessCount {
    count: u32,
}

impl GuessCount {
    /// Creates a new `GuessCount` initialized to 0.
    /// 
    /// # Returns
    /// Returns a new `GuessCount` instance with the count set to 0.
    pub fn new() -> GuessCount {
        GuessCount { count: 0 }
    }

    /// Increments the guess count by 1.
    pub fn increment(&mut self) {
        self.count += 1;
    }

    /// Returns the current guess count.
    /// 
    /// # Returns
    /// Returns the current count as a `u32`.
    pub fn value(&self) -> u32 {
        self.count
    }
}

/// Enum to represent possible errors when handling guesses.
#[derive(Debug)]
pub enum GuessError {
    /// Represents an invalid guess outside the allowed range (1 to 100).
    InvalidRange,
    /// Represents an error when parsing the guess input.
    ParseError(ParseIntError),
    /// Represents a general invalid input error (e.g., non-numeric input).
    InvalidInput,
}

/// Generates a random number between `start` and `end` (inclusive).
/// 
/// # Arguments
/// * `start` - The lower bound of the range (inclusive).
/// * `end` - The upper bound of the range (inclusive).
/// 
/// # Returns
/// Returns a `u32` value representing the random number.
fn get_secret_number(start: u32, end: u32) -> u32 {
    rand::thread_rng().gen_range(start..=end)
}

/// Prompts the user for a guess and returns a `Result` containing the `Guess` object or an error.
/// 
/// # Returns
/// Returns a `Result<Guess, GuessError>`:
/// - `Ok(Guess)` if the user input is valid and within range.
/// - `Err(GuessError::ParseError)` if the input cannot be parsed as a number.
/// - `Err(GuessError::InvalidRange)` if the guess is outside the valid range.
fn get_guess() -> Result<Guess, GuessError> {
    println!("Please input your guess:");

    let mut guess_str: String = String::new();

    io::stdin()
        .read_line(&mut guess_str)
        .expect("Failed to read line");

    let guess: u32 = match guess_str.trim().parse() {
        Ok(num) => num,
        Err(e) => return Err(GuessError::ParseError(e)),
    };

    Guess::new(guess)
}

/// Compares the user's guess with the secret number and prints the result.
/// 
/// # Arguments
/// * `guess` - The user's guess.
/// * `secret_number` - The randomly generated secret number to be guessed.
/// * `guess_count` - The number of guesses the user has made so far.
/// 
/// # Returns
/// Returns `true` if the user guessed correctly, otherwise `false`.
fn handle_guess(guess: u32, secret_number: u32, guess_count: u32) -> bool {
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small");
            false
        },
        Ordering::Greater => {
            println!("Too big");
            false
        },
        Ordering::Equal => {
            println!("You win, in {} guesses!", guess_count);
            true
        },
    }
}

/// Handles the display of error messages based on the provided `GuessError`.
/// 
/// # Arguments
/// * `error` - The `GuessError` that occurred.
/// 
/// # Example
/// ```
/// handle_error(GuessError::InvalidRange);
/// ```
fn handle_error(error: GuessError) {
    match error {
        GuessError::InvalidRange => {
            println!("Error: The number must be between 1 and 100.");
        },
        GuessError::ParseError(_) => {
            println!("Error: Please enter a valid number.");
        },
        GuessError::InvalidInput => {
            println!("Error: Invalid input, please try again.");
        },
    }
}

/// The main function that runs the game loop for guessing the secret number.
/// 
/// # Flow
/// The function will generate a random number, ask the user to input guesses,
/// and compare each guess against the secret number. It continues until the user guesses correctly.
/// 
/// # Example
/// ```
/// main();
/// ```
fn main() {
    println!("Guess the number");

    let secret_number: u32 = get_secret_number(1, 100);
    let mut guess_count = GuessCount::new();

    loop {
        let guess: Guess = match get_guess() {
            Ok(g) => g,
            Err(err) => {
                handle_error(err);
                continue;
            },
        };

        guess_count.increment();

        if handle_guess(guess.value(), secret_number, guess_count.value()) {
            break;
        }
    }
}

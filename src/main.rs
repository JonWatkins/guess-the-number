use guessing_game::{ErrorHandler, Guess, GuessCount, GuessResult, Incrementable};

/// The main function that runs the game loop for guessing the secret number.
///
/// This is the entry point of the "Guess the Number" game. The function handles the entire
/// game flow, including generating a secret number, prompting the user for guesses,
/// and comparing each guess to the secret number. The game continues until the user guesses
/// correctly, at which point it prints a victory message and ends.
///
/// # Flow
/// 1. The game generates a random secret number between 1 and 100 using the `get_secret_number` function.
/// 2. It initializes a `GuessCount` to track the number of guesses the player has made.
/// 3. The game enters a loop where it:
///    - Prompts the user to input a guess using the `get_guess` function.
///    - Validates and parses the input, handling any errors (e.g., invalid input or out-of-range guesses).
///    - Compares the guess to the secret number using the `handle_guess` function.
///    - Increments the guess count with each attempt.
/// 4. The loop continues until the user guesses correctly, at which point a success message is printed
///    and the game ends.
fn main() {
    println!("Guess the number");

    // Generate a random secret number between 1 and 100.
    let secret_number = Guess::new(guessing_game::get_secret_number(1, 100))
        .expect("Failed to generate secret number");

    // Initialize the guess count to track the number of attempts.
    let mut guess_count = GuessCount::new();

    // Game loop: continue until the user guesses correctly.
    loop {
        // Get the user's guess and handle any errors (invalid input or parsing errors).
        let guess = match guessing_game::get_guess() {
            Ok(g) => g,
            Err(err) => {
                // Handle input error (e.g., out of range or invalid input).
                err.handle_error();
                continue; // Ask for a new guess if there was an error.
            }
        };

        // Increment the guess count after each guess.
        guess_count.increment();

        // Compare the guess to the secret number and check if the user wins.
        match guessing_game::handle_guess(guess, &secret_number) {
            GuessResult::TooSmall => println!("Too small"),
            GuessResult::TooBig => println!("Too big"),
            GuessResult::Correct => {
                println!("You win, in {} guesses!", guess_count.value());
                break;
            }
        }
    }
}

use rand::Rng;
use std::{cmp::Ordering, io, num::ParseIntError};

/// Trait for handling errors in a modular and consistent way.
/// 
/// The `ErrorHandler` trait provides a mechanism for handling errors in a structured
/// and reusable manner. It defines a method `handle_error` which can be implemented
/// by types that need custom error handling logic. This allows error handling to
/// be modular, making the codebase more maintainable and consistent when dealing with errors.
/// 
/// Implementing this trait enables types to define how they handle errors, 
/// improving code clarity and reducing redundancy in error management across the application.
/// 
/// # Methods
/// 
/// ## `handle_error(&self)`
/// 
/// A method that handles the error associated with the type implementing the trait.
/// The implementation should define how the error should be processed or displayed.
/// 
/// # Use Case
/// This trait is especially useful when we want to define a consistent way of handling errors
/// across different types, such as custom error types or even standard library errors like `io::Error`.
/// For instance, it can be used in a game or application to handle specific error types like
/// invalid inputs, network failures, or unexpected conditions.
///
/// # Benefits
/// - **Modular error handling**: Separate the logic for handling different error types.
/// - **Consistency**: Ensures that all error-handling logic is standardized across the application.
pub trait ErrorHandler {
    /// Handles the error associated with the implementing type.
    ///
    /// This method should define how errors of this type should be processed,
    /// logged, or displayed. The specific behavior is left up to the implementation.
    fn handle_error(&self);
}

/// Enum to represent possible errors when handling guesses.
/// 
/// The `GuessError` enum defines the various errors that can occur when handling
/// user input or processing guesses in the guessing game. Each variant represents
/// a different type of error that can occur during the guessing process, from invalid
/// input to out-of-range guesses. This enum is used to provide clear error reporting
/// and handle different types of errors in a structured manner.
/// 
/// # Variants
/// 
/// ## `InvalidRange`
/// 
/// Represents an error that occurs when the user's guess is outside the valid range.
/// The valid range for guesses is between 1 and 100, inclusive. This variant is returned
/// when a guess is made that falls outside this range.
/// 
/// ## `ParseError(ParseIntError)`
/// 
/// Represents an error that occurs when parsing the user's input into a valid `u32`.
/// This variant contains the original `ParseIntError` returned when attempting to convert
/// a non-numeric string into a number. This error may occur if the user enters a non-numeric
/// value or improperly formatted input.
/// 
/// ## `InvalidInput`
/// 
/// Represents a general invalid input error. This variant is used for situations where
/// the input doesn't conform to the expected format but is not necessarily a parsing error.
/// It can be used for cases like empty input or special characters that aren't valid in a guess.
#[derive(Debug, PartialEq)]
pub enum GuessError {
    InvalidRange,
    ParseError(ParseIntError),
    InvalidInput,
}

/// Handles the display of error messages based on the provided `GuessError`.
/// 
/// This method is an implementation of the `ErrorHandler` trait for the `GuessError` enum.
/// It prints an appropriate, user-friendly error message to the console depending on which variant
/// of `GuessError` was encountered. The method provides clear feedback to the user, helping them
/// understand the nature of the error and how to resolve it.
///
/// # Arguments
/// * `self` - The `GuessError` variant that occurred. This variant determines which error message is printed.
///
/// # Behavior
/// - For `GuessError::InvalidRange`, it prints an error message indicating that the guess is outside
///   the valid range (1 to 100).
/// - For `GuessError::ParseError`, it indicates that the input could not be parsed into a valid number.
/// - For `GuessError::InvalidInput`, it prints a more general error message, asking the user to try again.
impl ErrorHandler for GuessError {
    fn handle_error(&self) {
        match self {
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
}

/// Trait for types that can parse user input into valid values.
/// 
/// The `Parsable` trait provides a standardized way to convert a string input
/// into a valid type. Types that implement this trait define how to parse a given
/// string (`&str`) and handle potential errors, returning a `Result` with either
/// the successfully parsed value or an error. This trait allows for flexible input
/// parsing and error handling across different types.
///
/// Types that implement `Parsable` can be used to consistently parse and validate user input
/// in a variety of contexts, such as in a game or command-line application, ensuring that
/// input is correctly parsed or appropriate error messages are returned.
///
/// # Associated Function
/// 
/// ## `parse_input(input: &str) -> Result<Self, GuessError>`
/// 
/// Attempts to parse the provided string into the implementing type. If the input is valid,
/// it returns `Ok(self)`. Otherwise, it returns a `GuessError` indicating what went wrong.
///
/// - **`input`**: A string slice (`&str`) containing the user input to be parsed.
/// - **Returns**: A `Result`:
///   - `Ok(Self)` if the parsing was successful.
///   - `Err(GuessError)` if the parsing failed. This error could be a `ParseError`, `InvalidRange`, or 
///      other types of errors depending on the implementation.
pub trait Parsable {
    /// Parses a string input into a valid value of the implementing type.
    /// 
    /// # Arguments
    /// * `input` - A string slice (`&str`) to be parsed into the implementing type.
    ///
    /// # Returns
    /// - `Ok(Self)` if the parsing is successful.
    /// - `Err(GuessError)` if the input is invalid, where the error could be a parsing error or some 
    ///    other validation failure.
    fn parse_input(input: &str) -> Result<Self, GuessError> where Self: Sized;
}

/// A trait for guessable objects.
/// 
/// The `Guessable` trait provides a common interface for objects that can be "guessed" in the
/// context of a guessing game. Types that implement this trait must define how they compare 
/// themselves to another object of the same type. This trait is useful for defining objects like
/// `Guess` or `secret_number` that need to be compared during gameplay, such as determining whether
/// a guess is too high, too low, or correct.
/// 
/// Implementing this trait allows various types to participate in the guessing game by providing
/// a standardized comparison operation, which is essential for determining the outcome of a guess.
/// 
/// # Associated Function
/// 
/// ## `compare(&self, other: &Self) -> Ordering`
/// 
/// Compares two objects of the same type and returns an `Ordering` that indicates whether the first
/// object is less than, greater than, or equal to the second object. The exact comparison logic depends
/// on the implementing type.
pub trait Guessable {
    /// Compares two objects of the same type.
    /// 
    /// This method compares the current object (`self`) with another object of the same type (`other`)
    /// and returns an `Ordering`. The result will be:
    /// - `Ordering::Less` if the current object is considered "smaller" than the other.
    /// - `Ordering::Greater` if the current object is considered "larger" than the other.
    /// - `Ordering::Equal` if the two objects are considered equal.
    ///
    /// The exact comparison logic is determined by the implementing type.
    fn compare(&self, other: &Self) -> std::cmp::Ordering;
}

/// A struct representing a guess made by the user.
/// 
/// The `Guess` struct stores a single guess made by the user in a guessing game. It contains a `u32`
/// value that represents the guessed number. The guess is typically within a predefined range (usually
/// between 1 and 100), but this range can be customized. The struct provides methods to create a new guess,
/// retrieve its value, and check if the guess is valid.
/// 
/// The `Guess` struct can be used in any context that involves guessing numbers, such as in games or quizzes
/// where players are asked to make a numeric guess and the system checks if the guess is correct.
/// 
/// # Fields
/// 
/// - `value`: The numeric value of the user's guess, stored as a `u32`.
#[derive(Debug, PartialEq)]
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

/// Trait implementation for `Guess` to make it "guessable".
/// 
/// This trait allows the `Guess` struct to be compared to another `Guess` object
/// using the `compare` method, which returns an `Ordering` based on the value of the guess.
/// 
/// The `compare` method compares the value of the current `Guess` (`self`) to the value of
/// another `Guess` (`other`). It returns an `Ordering` (`Less`, `Greater`, or `Equal`) depending
/// on whether the current guess is smaller, larger, or equal to the other guess.
impl Guessable for Guess {
    /// Compares the value of the current guess (`self`) to another guess (`other`).
    /// 
    /// # Arguments
    /// * `other` - A reference to another `Guess` instance to compare with.
    /// 
    /// # Returns
    /// Returns an `Ordering`:
    /// - `Ordering::Less` if `self.value` is less than `other.value`.
    /// - `Ordering::Greater` if `self.value` is greater than `other.value`.
    /// - `Ordering::Equal` if both values are equal.
    fn compare(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

/// Trait implementation for `Guess` to make it parsable from a string input.
///
/// This trait allows a `Guess` object to be created from a string input.
/// The input is first trimmed of any whitespace, then parsed as a `u32`
/// value. If the parsing is successful, the `Guess::new` method is used
/// to create a new `Guess` object. If any errors occur (e.g., the input
/// is not a valid number or it's out of the valid range for guesses), 
/// the error is mapped to a `GuessError::ParseError`.
///
/// # Arguments
/// * `input` - A string slice (`&str`) representing the input to be parsed into a `Guess`.
///
/// # Returns
/// Returns a `Result<Guess, GuessError>`:
/// - `Ok(Guess)` if the input is valid and within the allowed range (1 to 100).
/// - `Err(GuessError::ParseError)` if the input cannot be parsed into a `u32`.
/// - `Err(GuessError::InvalidRange)` if the parsed `u32` is outside the valid range.
impl Parsable for Guess {
    fn parse_input(input: &str) -> Result<Guess, GuessError> {
        let guess = input.trim().parse::<u32>().map_err(|e| GuessError::ParseError(e))?;
        Guess::new(guess)
    }
}

/// Trait for types that can be incremented.
/// 
/// The `Incrementable` trait defines a contract for types that represent a counter or a
/// value that can be incremented by a fixed amount. This trait provides a standard way to
/// increment such values, ensuring consistency across different types that need to support
/// incrementing operations. Types that implement this trait should define the logic for
/// how their values are incremented (e.g., increasing a counter by 1).
/// 
/// Implementing this trait allows any type to be used in contexts where an incrementable
/// value is required, such as counting iterations, tracking scores, or maintaining
/// any kind of cumulative total.
///
/// # Associated Function
/// 
/// ## `increment(&mut self)`
/// 
/// Increments the value of the type by a fixed amount (typically 1). The exact behavior
/// of the increment operation is determined by the implementing type.
pub trait Incrementable {
    /// Increments the value of the implementing type.
    /// 
    /// This method modifies the internal state of the type by incrementing its value.
    /// The exact increment logic is defined by the type implementing this trait.
    fn increment(&mut self);
}

/// A struct representing the number of guesses made by the user.
///
/// The `GuessCount` struct keeps track of how many guesses the user has made in a guessing game.
/// It provides methods to initialize the guess count, increment the count with each guess, and retrieve
/// the current count of guesses. This struct is useful for tracking the progress of a user in games
/// where the number of attempts is important, such as in a "Guess the Number" game.
///
/// # Fields
/// - `count`: A `u32` that holds the current number of guesses made by the user.
#[derive(Debug, PartialEq)]
pub struct GuessCount {
    count: u32,
}

impl GuessCount {
    /// Creates a new `GuessCount` initialized to 0.
    ///
    /// This method creates and returns a new instance of `GuessCount` with the `count`
    /// field set to 0. It is typically used to initialize the count before any guesses
    /// have been made.
    ///
    /// # Returns
    /// Returns a new `GuessCount` instance where the `count` is initialized to 0.
    pub fn new() -> GuessCount {
        GuessCount { count: 0 }
    }

    /// Returns the current guess count.
    ///
    /// This method returns the current value of the `count` field, representing
    /// how many guesses have been made so far in the game.
    ///
    /// # Returns
    /// Returns the current `count` as a `u32`. This value represents the total
    /// number of guesses made up to that point.
    pub fn value(&self) -> u32 {
        self.count
    }
}

/// Trait implementation for `GuessCount` to make it incrementable.
///
/// This trait allows the `GuessCount` struct to increment its `count` field by 1
/// each time the `increment` method is called. This is useful for tracking the 
/// number of guesses made by the user during the game.
impl Incrementable for GuessCount {
    /// Increments the `count` field of `GuessCount` by 1.
    ///
    /// This method increases the current count (the number of guesses made so far) by 1.
    /// It is typically called after each guess is made to keep track of the total number of guesses.
    fn increment(&mut self) {
        self.count += 1;
    }
}

/// Generates a random number between `start` and `end` (inclusive).
/// 
/// This function generates a random number within a specified inclusive range,
/// using the `rand::thread_rng()` function from the `rand` crate to access a
/// random number generator. The number generated is within the bounds specified
/// by the `start` and `end` parameters, including both `start` and `end`.
/// 
/// # Arguments
/// * `start` - The lower bound of the range (inclusive), as a `u32`. This is the smallest value that can be returned.
/// * `end` - The upper bound of the range (inclusive), as a `u32`. This is the largest value that can be returned.
/// 
/// # Returns
/// Returns a `u32` value representing the random number generated within the range `[start, end]`.
/// 
/// # Notes
/// - The `start` value must be less than or equal to the `end` value.
/// - This function relies on the `rand::thread_rng()` function from the `rand` crate to ensure secure randomness.
/// 
/// # Panics
/// This function will panic if `start` is greater than `end`, as the range is invalid.
pub fn get_secret_number(start: u32, end: u32) -> u32 {
    rand::thread_rng().gen_range(start..=end)
}

/// Prompts the user for a guess and returns a `Result` containing the `Guess` object or an error.
/// 
/// This function prompts the user to input a guess via the console, reads the input as a string,
/// and attempts to parse it into a valid `Guess`. If the input is valid and within the allowed range,
/// it returns an `Ok(Guess)`. If the input is invalid or outside the valid range, it returns
/// an appropriate error (`ParseError` or `InvalidRange`).
///
/// # Returns
/// Returns a `Result<Guess, GuessError>`:
/// - `Ok(Guess)` if the user input is valid and within the range of 1 to 100.
/// - `Err(GuessError::ParseError)` if the input cannot be parsed as a valid `u32`.
/// - `Err(GuessError::InvalidRange)` if the parsed guess is outside the valid range (1 to 100).
///
/// # Errors
/// This function may return the following errors:
/// - `GuessError::ParseError`: If the input is not a valid number (e.g., non-numeric input).
/// - `GuessError::InvalidRange`: If the parsed number is outside the valid range of 1 to 100.
pub fn get_guess() -> Result<Guess, GuessError> {
    println!("Please input your guess:");

    let mut guess_str: String = String::new();

    io::stdin()
        .read_line(&mut guess_str)
        .expect("Failed to read line");

    Guess::parse_input(&guess_str)
}

/// Enum to represent the result of a user's guess.
/// 
/// The `GuessResult` enum defines the possible outcomes when a user guesses a number.
/// It is used to provide clear feedback on whether the guess is too small, too large, 
/// or correct.
/// 
/// # Variants
/// 
/// - `TooSmall`: Indicates the guess is too small compared to the secret number.
/// - `TooBig`: Indicates the guess is too large compared to the secret number.
/// - `Correct`: Indicates the guess is equal to the secret number.
#[derive(Debug, PartialEq)]
pub enum GuessResult {
    TooSmall,
    TooBig,
    Correct,
}

/// Compares the user's guess with the secret number and returns the result.
/// 
/// This function compares the provided `guess` with the `secret_number` and returns
/// a `GuessResult` indicating whether the guess is too small, too large, or correct.
/// It does not print anything to the console, but the result can be used to give
/// appropriate feedback to the user (e.g., by printing a message).
/// 
/// # Arguments
/// 
/// * `guess` - The user's guess, which must implement the `Guessable` trait. The `guess` 
///   is compared to the `secret_number` to determine the result.
/// 
/// * `secret_number` - The randomly generated secret number to be guessed, which also
///   must implement the `Guessable` trait. The `secret_number` is compared with the
///   `guess` to determine whether it is too small, too big, or correct.
/// 
/// # Returns
/// 
/// Returns a `GuessResult`:
/// 
/// - `GuessResult::TooSmall` if the guess is smaller than the secret number.
/// - `GuessResult::TooBig` if the guess is larger than the secret number.
/// - `GuessResult::Correct` if the guess matches the secret number.
///
/// # Notes
/// 
/// The function assumes that both `guess` and `secret_number` are of types that implement
/// the `Guessable` trait, which defines the `compare` method for comparing the two values.
/// The result of the comparison is then used to determine which of the `GuessResult` variants
/// should be returned.
pub fn handle_guess<G: Guessable>(guess: G, secret_number: &G) -> GuessResult {
    match guess.compare(secret_number) {
        Ordering::Less => GuessResult::TooSmall,
        Ordering::Greater => GuessResult::TooBig,
        Ordering::Equal => GuessResult::Correct,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for parsing a valid input string into a Guess
    #[test]
    fn parse_input_valid() {
        let valid_input = "42";  // Valid input as a string
        let result = Guess::parse_input(valid_input);  // Parse the input
        // Check if the result is Ok, meaning the input was valid
        assert!(result.is_ok(), "Valid input should result in a valid Guess");
        // Check if the parsed Guess value is correct
        assert_eq!(result.unwrap().value(), 42);
    }

    // Test for handling invalid input that is not a number
    #[test]
    fn parse_input_invalid_number() {
        let invalid_input = "not_a_number";  // Invalid input string
        let result = Guess::parse_input(invalid_input);  // Try to parse the invalid input
        // Check if the result is Err, meaning the input could not be parsed
        assert!(result.is_err(), "Invalid input should result in an error");
        // Specifically check for a ParseError, although we don't care about the exact details here
        if let Err(GuessError::ParseError(_)) = result {
            // No additional assertions needed; we just want to check for the error
        } else {
            panic!("Expected ParseError");  // Panic if the error type is not what we expect
        }
    }

    // Test for handling input that is out of the acceptable range (1 to 100)
    #[test]
    fn parse_input_out_of_range() {
        let out_of_range_input = "150";  // Input exceeds the valid range
        let result = Guess::parse_input(out_of_range_input);  // Try to parse the input
        // Check if the result is an Err with InvalidRange error
        assert_eq!(result, Err(GuessError::InvalidRange));
    }

    // Test for creating a Guess with a valid value within the range
    #[test]
    fn guess_creation_valid_range() {
        let guess = Guess::new(50);  // Valid guess value
        // Check if the guess creation was successful
        assert!(guess.is_ok(), "Valid guess should be created successfully");
    }

    // Test for creating a Guess with a value below the valid range
    #[test]
    fn guess_creation_invalid_range_low() {
        let guess = Guess::new(0);  // Guess value is too low (below 1)
        // Check if the result is Err with InvalidRange error
        assert_eq!(guess, Err(GuessError::InvalidRange), "Guess below 1 should be invalid");
    }

    // Test for creating a Guess with a value above the valid range
    #[test]
    fn guess_creation_invalid_range_high() {
        let guess = Guess::new(101);  // Guess value is too high (above 100)
        // Check if the result is Err with InvalidRange error
        assert_eq!(guess, Err(GuessError::InvalidRange), "Guess above 100 should be invalid");
    }

    // Test for handling a correct guess (the guess matches the secret)
    #[test]
    fn handle_guess_correct() {
        let guess = Guess::new(50).unwrap();  // Create a guess with value 50
        let secret = Guess::new(50).unwrap();  // Secret value is also 50
        let result = handle_guess(guess, &secret);  // Check the result of the guess
        // The guess is correct, so the result should be Correct
        assert_eq!(result, GuessResult::Correct);
    }

    // Test for handling a guess that is too small
    #[test]
    fn handle_guess_too_small() {
        let guess = Guess::new(30).unwrap();  // Create a guess with value 30
        let secret = Guess::new(50).unwrap();  // Secret value is 50
        let result = handle_guess(guess, &secret);  // Check the result of the guess
        // The guess is too small, so the result should be TooSmall
        assert_eq!(result, GuessResult::TooSmall);
    }

    // Test for handling a guess that is too large
    #[test]
    fn handle_guess_too_big() {
        let guess = Guess::new(70).unwrap();  // Create a guess with value 70
        let secret = Guess::new(50).unwrap();  // Secret value is 50
        let result = handle_guess(guess, &secret);  // Check the result of the guess
        // The guess is too big, so the result should be TooBig
        assert_eq!(result, GuessResult::TooBig);
    }

    // Test for the initial value of the guess count
    #[test]
    fn test_guess_count_initialization() {
        let guess_count = GuessCount::new();  // Create a new GuessCount object
        // The initial value of GuessCount should be 0
        assert_eq!(guess_count.value(), 0, "Initial guess count should be 0");
    }

    // Test for incrementing the guess count
    #[test]
    fn test_guess_count_increment() {
        let mut guess_count = GuessCount::new();  // Create a new mutable GuessCount object
        guess_count.increment();  // Increment the guess count
        // Check if the guess count was correctly incremented to 1
        assert_eq!(guess_count.value(), 1, "Guess count should be incremented to 1");

        guess_count.increment();  // Increment again
        // Check if the guess count was correctly incremented to 2
        assert_eq!(guess_count.value(), 2, "Guess count should be incremented to 2");
    }
}

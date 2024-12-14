# Guess the Number Game

This is a simple command-line "Guess the Number" game implemented in Rust. The goal of the game is to guess a randomly generated number between 1 and 100. The program provides feedback on whether the guess is too high or too low, and tracks the number of guesses made. This isn't meant to be the most efficient
code, I am using this as a fun way to learn the features of Rust. There are a lot of comments for the code mainly to help me remember what im doing.

## Table of Contents

- [Installation](#installation)
  - [Windows](#windows)
  - [macOS](#macos)
  - [Linux](#linux)
- [Building the Project](#building-the-project)
- [Running the Game](#running-the-game)
- [License](#license)

## Installation

### Windows

1. **Install Rust**:
   - Go to the official Rust website: https://www.rust-lang.org/learn/get-started
   - Download and run the installer for Windows (`rustup-init.exe`).
   - Follow the on-screen instructions to install Rust.
   
2. **Verify Installation**:
   - Open a terminal (Command Prompt or PowerShell) and type the following command:
     ```bash
     rustc --version
     ```
   - This should output the installed version of Rust.

3. **Install the Game**:
   - Clone the repository to your local machine using `git`:
     ```bash
     git clone https://github.com/JonWatkins/guess-the-number.git
     ```
   - Navigate to the project directory:
     ```bash
     cd guess-the-number
     ```

### macOS

1. **Install Rust**:
   - Open the Terminal and install Rust using the `rustup` installer:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Follow the on-screen instructions to install Rust.

2. **Verify Installation**:
   - After installation is complete, check the installed version of Rust by running:
     ```bash
     rustc --version
     ```

3. **Install the Game**:
   - Clone the repository:
     ```bash
     git clone https://github.com/JonWatkins/guess-the-number.git
     ```
   - Navigate to the project directory:
     ```bash
     cd guess-the-number
     ```

### Linux

1. **Install Rust**:
   - Open a terminal and run the following command to install Rust via `rustup`:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Follow the on-screen instructions to install Rust.

2. **Verify Installation**:
   - Check the Rust version after installation by running:
     ```bash
     rustc --version
     ```

3. **Install the Game**:
   - Clone the repository:
     ```bash
     git clone https://github.com/JonWatkins/guess-the-number.git
     ```
   - Navigate to the project directory:
     ```bash
     cd guess-the-number
     ```

## Building the Project

Once you have installed Rust and cloned the repository, you can build the project.

1. **Navigate to the Project Directory**:
   Ensure you're in the directory where the game code is located:
   ```bash
   cd guess-the-number
   ```

2. Build the Project: To compile the project, run the following command:
   ```bash
   cargo build --release
   ```
   This will compile the project in release mode. The executable will be placed in the 
   target/release directory.

## Running the game

After building the project, you can run the "Guess the Number" game on your system.

### Running on Windows, macOS, and Linux:

1. Navigate to the Project Directory: Make sure you are in the project folder 
   (where the Cargo.toml file is located):
   ```bash
   cd guess-the-number
   ```

2. Build the Project: First, you need to build the project in release mode:
   ```bash
   cargo build --release
   ```
   This will compile the project and optimize it for performance. The compiled executable 
   will be located in the target/release directory.

3. Run the Game: After building the project, run the game with the following command:
   * On Linux/macOS:
     ```bash
     ./target/release/guessing_game
     ```
   * On Windows:
     ```bash
     .\target\release\guessing_game.exe
     ```

4. Game Flow:
    * The game will prompt you to input a guess.
    * You have to guess a secret number between 1 and 100.
    * After each guess, the game will tell you if your guess is too high, too low, or correct.
    * The game will continue until you guess the correct number.
    * The number of guesses will be tracked, and once you win, the total number of guesses will be displayed.

5. Example Output:
   ```bash
   Guess the number
   Please input your guess:
   50
   Too small
   Please input your guess:
   75
   Too big
   Please input your guess:
   62
   Too small
   Please input your guess:
   68
   You win, in 4 guesses!
   ```


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

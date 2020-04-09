# Rust-Command-Line-Guessing-Game
A cool little command line guessing game built in Rust!

<br/>

## Inspired by Chapter 2 of The Rust Programming Language Book

You can find the book online [here](https://doc.rust-lang.org/book/), and specificaly the guessing game chapter [here](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)!

<br/>

## Improvements To The Book's Guessing Game

_Note: These are currently just ideas..._

Here are some ideas for features we could add that build on the final source code in the book and make the guessing game even more awesome!

[ &nbsp; ] - Display the number of guesses it took the user to get to the correct number.

[ &nbsp; ] - Display a "high score / personal best" number of guesses it took the user to get to the correct number.

[ &nbsp; ] - Display cool ansi art when the user guesses the wining number!

[ &nbsp; ] - If the program says lower and the user enters a higher number, print a funny message indicating higher than the higher of the two numbers.

[ &nbsp; ] - Allow the user to change the range by passing an arugment with `cargo run` (and use 100 as the default)


<br/>

## Usage Guide

In order to run this code you just need to have [cargo`](https://github.com/rust-lang/cargo) installed.

To check if you have cargo installed:
```
cargo -v
```

Then run the below commands inside the `guessing_game` directory:

<br/>

### Run Locally
```
cargo run
```

<br/>


## Scaffolding

This project in the `guessing_game` directory was initially created with cargo:
```
cargo new guessing_game
```
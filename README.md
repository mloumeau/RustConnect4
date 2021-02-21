# RustConnect4


# Overview

In this project, I created the logic of a Connect 4 game, and was able to display it in ASCII form for the user to play in the terminal.

I wrote this software as a way of learning the language Rust. I've bee having fun learning new languages recently, and I wanted to give Rust a go.
It is always hard for me to come up with ideas for projects when learning a language, but I thought Connect 4 was more of a challenge than Tic Tac Toe,
and not so challenging that it would be eating up a bunch of time.


[Software Demo Video](https://youtu.be/mP_ItmFsuY4)

# Development Environment

For this project, I needed to install Rust, the Rust extension in Visual Studio Code, and create a rust project through the command prompt.

For this project, I didn't need to import lots of libraries. The only library I needed was std. This allowed for me to use
std::io:stdin() for getting input from the user, and std::process:exit() to kill the program when necessary.

# Useful Websites

{Make a list of websites that you found helpful in this project}
* [Install Rust](https://www.rust-lang.org/tools/install)
* [Getting User Input (Tricky)](https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0)

# Future Work

* I'd like to create a better GUI, perhaps in an app with Android Studio.
* I wanted the game to draw when the last chip was inserted instead of making a move after the game is full, but that was getting too 
complicated for how unimportant that was.
* I could potentially find a way to check for wins with less code.

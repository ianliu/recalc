# recalc

I've made this project to learn a little bit about the Rust programming
language. It is a _reactive_ calculator, meaning that expressions are always
lazily evaluated. Take a look at the following example:

    > x = 10
    x = 10
    > y = 2 * x
    y = 20
    > x = 20
    x = 20
    > y
    40

## Getting Started

Clone this repo and run `cargo run`. You will be greeted with a REPL where you
can input commands. You can use

 * `+`, `-`, `*`, `/`, and `^` to perform calculations;
 * `pi` and `e` constants;
 * `sin`, `cos`, and `tan` functions;
 * `=` to define variables.

## Disclaimer

This is a toy project, there are bugs and they can bite you. For instance,
*don't* try this: `x = x`.

## Future

I would like to advance this project to include the following features:

 - [ ] Pin board, an area showing pinned variables
 - [ ] Array formulas

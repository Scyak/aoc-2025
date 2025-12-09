# Advent of Code repository template

This repository is my template for Rust AoC solutions. Don't know what Advent of Code is? Read more on [their website](https://adventofcode.com/).

## Prerequisites

If you want to use the dev container that comes with this repository (so you don't have to install a Rust toolchain etc. manually), you need to have Docker and the Visual Studio Code extension Dev Containers installed. Using the dev container is optional if you have your own Rust toolchain.

## Setup

If you want to use the repository as a template for your own solutions, follow these steps:  

1. Clone this repository
2. Use the python script in `util` to generate blank day files:
``` bash
python3 util/generate_days.py
```
3. If you want to use the dev container, open the repository folder in Visual Studio Code. When prompted (there should be a popup in the bottom right corner), select "Reopen in Container". If the prompt doesn't appear, click the blue dev container icon in the bottom left corner and select "Reopen in Container" from there.
4. If you want to automatically fetch your inputs from the AoC website, find your session token as described [here](https://github.com/wimglenn/advent-of-code-wim/issues/1) and put it in a file called .token in the repository root.

## How to run
To run your solution for the current day's problem, simply call `cargo run`. This will fetch today's input if there isn't a `day<day>.txt` file in `inputs`, and then run the current day.

Subcommands are available for running specific days (`cargo run -- run <day>` - this also fetches the input if it's not present) and for fetching inputs from the AoC website (`cargo run -- fetch <day>`).

You can always get a description of the subcommands by running `cargo run -- help`, or a description of their arguments by running `cargo run -- help <subcommand>`.

For computationally intensive solutions you may wish to call cargo run with the `--release` flag (rather than in the default debug mode) to speed up runtime: `cargo run --release [-- run <day>]`

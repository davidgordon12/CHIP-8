# CHIP-8
A Chip-8 emulator implemented in Rust with SDL2

This emulator does not feature sound, but is otherwise accurate.

# Usage

## Prerequisites
- cargo (Rust)
- SDL2
- Homebrew (optional)

## Getting started

SDL2 must first be installed. On mac, you can install it with (assuming that Homebrew is installed) <br />
``` brew install sdl2 ``` <br />
``` brew install sdl2_image ``` <br />
``` brew install sdl2_ttf ``` <br />


Clone the repo with <br />
``` git clone https://github.com/davidgordon12/chip-8 ``` <br />

Navigate into chip-8 and, in a terminal, enter
``` cargo run INVADERS ```

## Controls

    Keyboard                    Chip-8
    +---+---+---+---+           +---+---+---+---+
    | 1 | 2 | 3 | 4 |           | 1 | 2 | 3 | C |
    +---+---+---+---+           +---+---+---+---+
    | Q | W | E | R |           | 4 | 5 | 6 | D |
    +---+---+---+---+     =>    +---+---+---+---+
    | A | S | D | F |           | 7 | 8 | 9 | E |
    +---+---+---+---+           +---+---+---+---+
    | Z | X | C | V |           | A | 0 | B | F |
    +---+---+---+---+           +---+---+---+---+

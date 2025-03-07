# rust-music-keyboard

[![build](https://github.com/logancammish/rust-music-keyboard/actions/workflows/rust.yml/badge.svg)](https://github.com/logancammish/rust-music-keyboard/actions/workflows/rust.yml)

Simple musical keyboard made in Rust


If you want more up-to-date but unstable functionality, compile the preproduction branch.

| Feature                     | Availablity  |
|-----------------------------|----------------------------------------------------------------------------------------------|
| **MIDI integration and recording**        | 👷  |
| **Play all notes in western music**       | ✔️ |
| **Play multiple notes asynchronously**       | ❌ |
| **Adjust BPM/Octave**       | ✔️ |
| **Adjust note duration**       | ❌ |
| **Determine major scales of a note**       | ✔️ |
| **Play ascending 1st/3rd/5th triad appregios**       | ✔️ |
| **Highlight keys according to a selected major scale**       | ❌ |


### Building/Downloading

This application officially supports Windows and Linux. 
1. Ensure you have cargo installed, if not install with [Rustup](https://www.rust-lang.org/tools/install)
2. Clone this repository (`git clone https://github.com/logancammish/rust-music-keyboard.git`)
3. Build with `cargo build --release`
4. You will find the executable in `target/release`

Alternatively, windows users may download the latest binary (if available) [here](https://github.com/logancammish/rust-music-keyboard/releases/latest).

### Use as a Rust crate

This application is not intended for production use or use inside of a Rust crate; but it should work (suppose you need the `Song` struct, or other functionality).


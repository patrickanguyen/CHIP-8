# CHIP-8 Interpreter

> This project is still a WIP and is subject to change.

[CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) is an interpreted programming language first used on 8-bit computer systems in the mid 1970's. It was make video game development easier on these systems.

I developed this project mostly to learn more about Rust and developing emulators.

## Compiling and Running

1. Follow the following [instructions](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries) to install the SDL2 development libraries depending on your operating system.

2. Compile & Run:
```
cargo run --release
```

3. Run tests:
```
cargo test
```

## To Do List
- [ ] Implement all CHIP-8 instructions
- [ ] Refactor SDL-2 window code
- [ ] Command-line arguments
- [ ] Keyboard support
- [ ] Timer support
- [ ] Sound support

## License
This repository is licensed under the [MIT License](LICENSE.md)

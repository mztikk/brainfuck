# BRAINFUCK
Rust library built on top of [poetic](https://github.com/mztikk/poetic) to parse [brainfuck](https://esolangs.org/wiki/Brainfuck).

## Tests
There are tests for the parser and interpreter which you can run with:

```Rust
cargo test
```

## Usage
It will first parse the string source input and parse it to only contain brainfuck opcodes.

```Rust
let buf = fs::read_to_string("input.bf").unwrap();
let intermediate = Parser::parse_string(&buf);
```

These opcodes can then be turned into instructions

```Rust
let mut instructions = Parser::parse_instructions(&intermediate);
```

which can then be optimized and interpreted by [poetic](https://github.com/mztikk/poetic) see https://github.com/mztikk/poetic#usage

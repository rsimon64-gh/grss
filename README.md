# grrs

A command-line tool to search files, written in Rust.
Following the tutorial here: [https://rust-cli.github.io/book/tutorial/](Rust Client Tutorial)

## Installation

```bash
cargo install grrs
```

## Usage
```bash
grrs <pattern> <path>
```

### Where:

pattern: The text pattern to search for
path: The path to the file to search in

## Example

```bash
grrs "hello" test.txt
```

License
This project is licensed under 

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

## Development

This project uses Git hooks to ensure code quality. The pre-commit hook will:
- Check code formatting with `rustfmt`
- Run linting with `clippy`
- Execute all tests

The hooks are automatically installed when you clone the repository. If they're not executable, run:

[![Workflow Status](https://github.com/webern/cargo-readme/workflows/main/badge.svg)](https://github.com/webern/cargo-readme/actions?query=workflow%3A%22main%22)
[![crates.io](https://img.shields.io/crates/v/grss_clone.svg)](https://crates.io/crates/grss_clone)
[![codecov](https://codecov.io/gh/rsimon64-gh/grss/graph/badge.svg?token=O46IELCY5O)](https://codecov.io/gh/rsimon64-gh/grss)
[![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)](https://github.com/rsimon64-gh/grss)
[![Docs](https://docs.rs/grss_clone/badge.svg)](https://docs.rs/grss_clone)
[![Demo](https://img.shields.io/badge/demo-online-brightgreen)](https://your-demo-link.com)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


# grss_clone

[Initial tutorial](https://rust-cli.github.io/book/index.html)

grss_clone is a command line search tool that finds occurrences of a pattern in a file.

## Features

- Search for text patterns in files
- Search using regular expressions
- Line-by-line output of matches

## Performance

The tool reads files line by line to handle large files efficiently without
loading the entire file into memory.

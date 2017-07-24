## README

Implementation of the [Huffman coding](https://en.wikipedia.org/wiki/Huffman_coding) compression algorithm in Rust

## todo

 - Improve code coverage, it's way too low right now.
 - Provide a command line interface with [clap](https://clap.rs/) or similar
 - Cleanup the code (dirty commented out prints, useless HuffmanTable struct, etc)

## Setup

Install the git pre-commit hook in order to run the unit tests before a commit:

```bash
$ ln -s "$(pwd)/pre-commit.sh" .git/hooks/pre-commit
$ cargo test
```

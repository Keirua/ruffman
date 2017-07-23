## README

Implementation of the [Huffman coding](https://en.wikipedia.org/wiki/Huffman_coding) compression algorithm in Rust

## todo

Make it work :) Compression seems to work as expected, but the compressed text is not

 - Improve code coverage. It's quite scary right now.
 - Debug the decompress function, which does not behave as expected
 - Provide a command line interface with [clap](https://clap.rs/) or similar
 - Cleanup the code (dirty commented out prints, useless HuffmanTable struct, etc)

## Setup

Install the git pre-commit hook in order to run the unit tests before a commit:

```bash
$ ln -s "$(pwd)/pre-commit.sh" .git/hooks/pre-commit
$ cargo test
```

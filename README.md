# aoc-2020

![Rust](https://github.com/tardisman5197/aoc-2020/workflows/Rust/badge.svg?branch=main)

Using 2020's Advent of Code to learn Rust. Hopefully I will get through the majority of the days.

## Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)

## Building

```
cargo build --release
```

## Testing

To run tests for all of the days:

```
cargo test
```

To test only one day:

```
cargo test -p <day-x>
```

e.g.

```
cargo test -p day-1
```

## Running

To run all of the days:

```
cargo run
```

To run only one day:

```
cargo run --release -p <day-x>
```

e.g.

```
cargo run --release -p day-1
```

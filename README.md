# Vivid Time Rust Libraries and Utilities

This project provides tools for working with **Vivid Time**, the time and calendar system I created around 2008, and have been using in several ways since. I will put an explanation of the time system here later.

The project consists of three libraries and a CLI for dealing with **Vivid Time**. Two of the libraries, *numburs* and *mixed_point*, are used to construct the number systems and calculations used in *vivid_time*. This is not, by any means, the simplest way of implementing the time systems, but I wanted to use this project to experiment with some numerical abstractions implemented in *numburs* and *mixed_point*.

## Components

### Numburs

This library implements abstracted integer and floating point operations with types that enforce properties of the underlying data.

### Mixed Point

This library implements cycle-based operations built from heterogeneous periods.

### Vivid Time

This library implements the actual time and calendar system for **Vivid Time**.

### Vivid Time CLI

This executable provides a CLI wrapper around operations in the Vivid Time library. The `--help` flag should explain most of it.

## Development and Building

Using Rust 1.82, this can be built with

```bash
cargo build --release
```
creating the executable in `target/release/vivid_time`.

To test and lint the project, run
```bash
cargo nextest run && cargo clippy
```


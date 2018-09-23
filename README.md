# `rdu`: a `du` ASCII visualizer in Rust #

[![build status](https://gitlab.com/andeh575/rdu/badges/master/build.svg)](https://gitlab.com/andeh575/rdu/commits/master)
[![rather rusty](https://img.shields.io/badge/rather-rusty-%23B7410E.svg)](https://www.rustup.rs/)

Copyright © 2018 Andrew Graham

This is a new implementation of [duvis](https://github.com/BartMassey/duvis), an `xdu` replacement for visualizing `du` disk usage output, written in Rust. `duvis` had an unfortunate dependency on `gtk+-3.0` for the graphical mode and `rdu` seeks to limit external dependencies to ensure cross-platform operability.

## Building ##

`rdu` can (hypothetically) be fine building with the latest stable rust:

```sh
# Ensure rust is up-to-date
rustup update

# Build
cargo build
```

### Testing ###

`rdu` comes with some unit tests that can be executed as follows:

```sh
cargo test
```

## Usage ##

`rdu` consumes the output of `du`; which is read for standard input (so a pipe or a file is acceptable).

```sh
du | rdu
```

### Options ###

+ `-p`, `--pre-order`: Enable pre-order sorting
+ `-v`, `--verbose`: Enable verbose output
+ `-d`, `--debug`: Enable debug printing

## License ##

This software provided under the [MIT License](https://opensource.org/licenses/MIT).

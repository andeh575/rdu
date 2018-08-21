# `rdu`: a `du` ASCII visualizer in Rust #

[![rather rusty](https://img.shields.io/badge/rather-rusty-%23B7410E.svg)](https://www.rustup.rs/)

Copyright © 2018 Andrew Graham

This is a new implementation of [duvis](https://github.com/BartMassey/duvis), an `xdu` replacement for visualizing `du` disk usage output, written in Rust.

## Usage ##

`rdu` consumes the output of `du`; which is read for standard input (so a pipe or a file is acceptable).

```sh
du | rdu
```

### Options ###

+ `-p`, `--pre-order`: Enable pre-order sorting

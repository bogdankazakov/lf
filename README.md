# lf

`lf` is a program to filter logs stream.
It's simple and easy to use.

[Installation](#installation) • [How to use](#how-to-use)

## Features

* scroll using keys or mouse
* toggle view of input field and scrollbar
* autoscroll (show new log entries)

## Demo

![Demo](doc/screencast.svg)

## How to use

There are two options:

* with subprocess (recommended) `lf ping yandex.ru`
* in pipe (nushell example) `ping yandex.ru e+o>| lf`

To get an overview of all available options just press `Ctr+h`

```
Use `Ctr` +:
    `c/q` -> quit
    `b` -> toggle scrollbar
    `s` -> toggle search input
    `h` -> toggle help
    `u/d` -> scroll up/down (turns off autoScroll)
    `p/n` -> page up/down (turns off autoScroll)
    `a` -> turn on autoScroll
    `t` -> scroll to the top
```

## Installation

1. Install Rust
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. Clone repo

3. Build app
`cargo install --locked --path .`

## License

`lf` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.

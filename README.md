<div align="center">
<h1>zman</h1>

<a href="https://builds.sr.ht/~azzamsa/zman?">
<img src="https://builds.sr.ht/~azzamsa/zman.svg">
</a>
<a href="https://crates.io/crates/zman">
<img src="https://img.shields.io/crates/v/zman.svg">
</a>
<a href=" https://docs.rs/zman/">
<img src="https://docs.rs/zman/badge.svg">
</a>
<a href="https://azzamsa.com/support/"><img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4"></a>
<p></p>

![demo](docs/demo.gif)

</div>

---

**zman** is a CLI year (time) progress that small, fast, and just one single binary.

## Features

- Show year progress
- Show month, and week progress
- Show result in JSON format
- Cross-platform

## Usage Examples

``` bash
zman                                Show a year progress bar
zman month                          ... a month progress bar
zman week                           ... a week progress bar
zman year --json                    ... a year progress bar with JSON format
```

## Usage with other tools

You can use Zman with i3status-rs to show salah time in your status bar.

![i3status-zman](docs/i3rs.png)

i3status-rs configuration Example:

``` bash
[[block]]
block = "custom"
cycle = ["zman year -J", "zman month -J"]
on_click = "<command>"
interval = 300
json = true
```

## Installation

### From binaries

The [release page](https://git.sr.ht/~azzamsa/zman/refs/) includes
pre-compiled binaries for GNU/Linux, macOS and Windows.

### From source

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

``` bash
cargo install zman
```

## Development

``` bash
git clone https://git.sr.ht/~azzamsa/zman
cd zman

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

## Contributing

For reporting issues, visit the tracker here: https://todo.sr.ht/~azzamsa/Zman

Please send patches to `~azzamsa/public-inbox@lists.sr.ht`

### Unsure how to contribute?

- [How to send patches](https://git-send-email.io/#step-3)

## Origin of the name

Zman is a 'time' in Arabic/Turksih.

## Inspired By

Please check out this previous work that helped inspire the creation of zman.

- [@year_progress](https://twitter.com/year_progress) - The only progress bar you'd rather see go slower. [source](https://github.com/filiph/progress_bar)

## License

Copyright (c) 2020 Azzamsa

Bilal is distributed under the terms of [GPL V3 License](LICENSE).



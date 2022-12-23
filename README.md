<div align="center">
<h1>zman</h1>

<a href="https://github.com/azzamsa/zman/workflows/ci.yml">
<img src="https://github.com/azzamsa/zman/workflows/ci/badge.svg">
</a>
<a href="https://crates.io/crates/zman">
<img src="https://img.shields.io/crates/v/zman.svg">
</a>
<a href="https://docs.rs/zman/">
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

```bash
zman                                Show a year progress bar
zman month                          ... a month progress bar
zman week                           ... a week progress bar
zman year --json                    ... a year progress bar with JSON format
```

## Integration with other programs

### Using zman with `i3status-rust`

You can use Zman with i3status-rust to show time period in your status bar.

![i3status-rust-zman](docs/i3status-rust.png)

i3status-rust configuration Example:

```bash
[[block]]
block = "custom"
cycle = ["zman year -J", "zman month -J"]
on_click = "<command>"
interval = 300
json = true
```

### Other tools

See [more examples](examples/) to learn other variations.

If you like `zman` to support your favorite status-bar, please open the issue
with the valid input of your status-bar. In i3status-rust
the valid input it would be `{"icon": "ICON", "state": "STATE", "text": "YOURTEXT"}`.

## Installation

### From binaries

The [release page](https://github.com/azzamsa/zman/releases) includes
pre-compiled binaries for GNU/Linux, macOS and Windows.

### From source

With Rust's package manager [cargo](https://github.com/rust-lang/cargo), you can install fd via:

```bash
cargo install zman
```

## Development

```bash
git clone https://github.com/azzamsa/zman

# Build
cd zman
cargo build

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

## Contributing

To learn more read [contributing.md](docs/dev/contributing.md)

## Origin of the name

Zman is a 'time' in Arabic/Turksih.

## Inspirations

Please check out this previous work that helped inspire the creation of zman.

- [@year_progress](https://twitter.com/year_progress) - The only progress bar you'd rather see go slower. [source](https://github.com/filiph/progress_bar)

## License

Copyright (c) 2020-2022 azzamsa

`zman` is distributed under the terms of the GPL v3 License.

See the [LICENSE](LICENSE) for details.

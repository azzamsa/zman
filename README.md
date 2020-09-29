# Zman

**zman** is a CLI time progress that small, fast, and just one single binary.

![demo](https://git.sr.ht/~azzamsa/zman/blob/master/media/zman-demo.gif)

## Features

- Show year progress
- Show month, and week progress
- Show result in JSON format
- Cross-platform

## Installation

## With cargo (from source)

``` bash
$ git clone https://git.sr.ht/~azzamsa/zman
$ cd zman
$ cargo build --release
$ cp target/release/bilal /usr/local/bin/
```

## From binaries

Download the binary from the [Release](https://git.sr.ht/~azzamsa/zman.rs/refs/)

Available binaries:
- *Nix
- Window PC


## Usage with other tools

You can use Zman with i3status-rs to show salah time in your status bar.

![i3status-zman](https://git.sr.ht/~azzamsa/blobs/blob/master/zman/zman.png)

i3status-rs configuration Example:

``` bash
[[block]]
block = "custom"
cycle = ["zman -y -j", "zman -m -j"]
on_click = "<command>"
interval = 300
json = true
```
## Contributing

For reporting issues, visit the tracker here: https://todo.sr.ht/~azzamsa/Zman

Please send patches to `~azzamsa/zman-devel@lists.sr.ht`

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



# Zman

**zman** is a CLI time progress that small, fast, and just one single binary.

![demo](https://git.sr.ht/~azzamsa/blobs/blob/master/zman/zman.gif)

## Features

- Show year progress
- Show month, and week progress
- Show result in JSON format
- Cross-platform

## Installation

## With cargo (from source)

``` bash
$ git clone https://git.sr.ht/~azzamsa/zman.rs
$ cd zman.rs
$ cargo build --release
$ cp target/release/bilal /usr/local/bin/
```

## From binaries

Download the binary from the [Release](https://git.sr.ht/~azzamsa/zman.rs/refs/)


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

## Origin of the name

Zman is a 'time' in Arabic/Turksih.

## License

Copyright (c) 2020 Azzamsa

Bilal is distributed under the terms of [GPL V3 License](LICENSE).



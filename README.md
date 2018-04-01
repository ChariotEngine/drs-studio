# `drs-studio`

[![Build Status](https://travis-ci.org/ChariotEngine/drs-studio.svg?branch=master)](https://travis-ci.org/ChariotEngine/drs-studio) [![Build Status](https://ci.appveyor.com/api/projects/status/github/ChariotEngine/drs-studio?branch=master&svg=true)](https://ci.appveyor.com/project/ChariotEngine/drs-studio) [![GitHub (pre-)release](https://img.shields.io/github/release/ChariotEngine/drs-studio/all.svg)](https://github.com/ChariotEngine/drs-studio/releases)

A tool for manipulating DRS archives.

This repository consists of a command-line application and a GTK-based UI.

See [ui/gtk/README.md](./ui/gtk/README.md) for information regarding the UI.

---

## What is a `DRS` archive?

Age of Empires uses a home-grown format ("DRS") to store palettes, sprites, and audio.

This format is conceptually similar to tar which contains other files but does not perform compression.

## Building

You must have the [Rust](https://rust-lang.org) toolchain installed (which includes `cargo`).

```sh
cargo build --release
```

The output binary will be written to `target/release/drs-studio`.

You can invoke this directly or put it somewhere on your shell's `$PATH`.

## Running

```sh
$ cargo run -- extract --drs-path /media/AOE/GAME/DATA/GRAPHICS.DRS --file-names 00412.slp
```

## License

[MIT](LICENSE)

## Contributing

Any contribution you intentionally submit for inclusion in the work, as defined
in the `LICENSE` file, shall be licensed as above.

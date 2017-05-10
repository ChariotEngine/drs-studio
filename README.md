# `drs-studio`

A command-line utility for manipulating DRS archives.

---

## What is a "DRS archive"?

Age of Empires uses a home-grown format ("DRS") to store palettes, sprites, and audio.

This format is conceptually similar to tar which contains other files but does not perform compression.

## License

MIT

## Usage

```sh
$ cargo run -- extract --drs-path /media/AOE/GAME/DATA/GRAPHICS.DRS --file-name 00412.slp
```

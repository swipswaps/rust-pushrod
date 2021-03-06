# rust-pushrod

## Attention

This project is being split into separate modules to make the project more manageable,
and as such will soon deprecate this project.

Pushrod is now split into:

- [Pushrod Events](https://www.github.com/KenSuenobu/pushrod-events)
- [Pushrod Widgets](https://www.github.com/KenSuenobu/pushrod-widgets)
- [Pushrod Core](https://www.github.com/KenSuenobu/pushrod)

This project will be shut down soon, but the tickets will be migrated over, as will the
widgets and some of the code.  Callbacks are being deprecated, as they are too complicated
to work as expected.  As a result, an event-based system is planned, and will soon be
explained and covered in a blog post.

## Project Description

[![Build Status](https://travis-ci.org/KenSuenobu/rust-pushrod.svg?branch=master)](https://travis-ci.org/KenSuenobu/rust-pushrod)
[![](https://img.shields.io/crates/d/rust-pushrod.svg)](https://crates.io/crates/rust-pushrod)
[![docs.rs for rust-pushrod](https://docs.rs/rust-pushrod/badge.svg)](https://docs.rs/rust-pushrod)

**Cross Platform UI Widget Library for Rust that uses SDL2.**

Draws inspiration from lots of GUI libraries.

If you like this library, [please consider donating to this project!](https://www.patreon.com/KenSuenobu)

## Philosophy

The reason I created this library instead of extending another library was that
I wanted to keep these specific design ideas in mind:

- Maintainable with little effort
- Easily extensible
- Lightweight enough to run on minimalist hardware
- **Easy to use and understand**

These design ideas are critical.  **Keep it simple.  Keep it stupid simple.**

[Click here to view my Blog!](https://kensuenobu.github.io/)

## Pushrod Widgets

[Click here to get a list of the Widgets available!](src/widgets/README.md)

## 0.4.x Status

Please [see here](https://github.com/KenSuenobu/rust-pushrod/milestone/5) for more details on issues.

## Prerequisites for Pushrod

Pushrod only requires:

| Library | Version |
| ------- | ------- |
| SDL2    | 0.33 |

### Ubuntu

```bash
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev
```

### Mac OS X

```bash
brew update
brew upgrade
brew install ruby
brew install sdl2 sdl2_image sdl2_ttf
```

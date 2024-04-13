
# <o[ Shellaga ]o>

A Galaga-like game which runs in the Terminal.

## ⚡ Quick Start ⚡

Simply checkout the code and run the target via cargo. 🦀

```shell
git clone git@github.com:BenLeadbetter/shellaga.git
cd shellaga
cargo run
```

## 🔧 Requirements 🔧

Currently shellaga requires a Terminal which supports [Kitty terminal protocol extensions](https://sw.kovidgoyal.net/kitty/protocol-extensions/).


## ✏️  Design ✏️

We use the [Bevy game engine](https://bevyengine.org/) and it's Entity Component System to handle game logic, run loop, time, etc.

We use [Crossterm](https://github.com/crossterm-rs/crossterm) to handle rendering in the terminal and terminal events.

We use [Ratatui](https://ratatui.rs/) to handle drawing ui.

## 🌟 Contribution 🌟

Contributions are actively encoraged!
Please check out the Issues for ideas.


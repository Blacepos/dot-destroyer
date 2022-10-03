# Dot Destroyer

This is a remake of a game I made in 2018 using the [LÖVE](https://love2d.org/) framework for Lua. This time, it's in Rust using [Bevy](https://bevyengine.org/).

## Building from source

If you wish to compile it yourself, you will have to download Rust: https://rustup.rs. The link will guide you to install the Rust compiler and build tools.

Once you have it installed, running it should be as simple as:
```sh
cargo run --release --bin=DotDestroyer
```
You can find the binary in target/release.

Note, however, that Bevy is a very large library and will likely take a long time to compile as well as require at least 1 GB of space.
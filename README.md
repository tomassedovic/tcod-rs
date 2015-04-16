libtcod bindings for Rust [![Build Status](https://travis-ci.org/tomassedovic/tcod-rs.svg?branch=master)](https://travis-ci.org/tomassedovic/tcod-rs)
-------------------------

[libtcod a.k.a. "The Doryen Library"](http://roguecentral.org/doryen/libtcod/)
is a smallish library designed for writing roguelikes. It provides a bunch of
useful functionality such as:

* Text-based graphics API that doesn't suck as much as Curses or OpenGL
* Colours! (like, more than 16)
* Keyboard and mouse input
* Path finding
* Field of view
* Portable (works on linux, windows and mac)
* [Lots of other stuff](http://roguecentral.org/doryen/libtcod/features/)


This project provides [Rust](http://www.rust-lang.org/) bindings for libtcod
v1.5.2.

This project follows [Semantic Versioning](http://semver.org/). Since we're
under `1.0.0` anything goes. The API can change at any time.

Indeed, it probably should change! If you have better ideas on how it make it
safer or more familiar to Rust developers, please let us know.


Documentation
---------------

We run `rustdoc` on every new commit:

http://tomassedovic.github.io/tcod-rs/tcod/index.html

But that's mostly useful for types, function signatures, etc. We don't have much
in term of actual docs, but you can always check the official ones:

http://roguecentral.org/doryen/data/libtcod/doc/1.5.1/index2.html?c=true


Current status
--------------

All raw tcod bindings are available via the `tcod-sys` crate. In addition we
want to provide safe (and more in line with the Rust style) wrappers -- if you
can segfault outside of `unsafe` blocks, that's a bug. The safe bindings are not
yet complete, however.

### Already Implemented

* _Colors_
* _Console_
* Most of the _System_ layer (FPS, time, fullscreen, screenshots)
* _Field of view_
* _Map_
* _Path finding_ (both A\* and Dijkstra)

### Probably Won't Ever Be Implemented Because Rust Provides This Already
* Filesystem utilities
* Containers
* Pseudorandom generator (Rust has good RNGs, but maybe we want to provide this
  anyway for people porting existing code depending on tcod's RNG to Rust?)
* Compression toolkit (there will probably be a better Rust library for this)

### Not Implemented Yet But Should Happen At Some Point In The Future
* Everything else!



How to use this
---------------

`tcod-rs` depends on `libtcod` so you need to build or download the official
version. The `libtcod` version known to work is bundled with `tcod-sys` and
Cargo will build it for you, but you need the build dependencies installed.

Alternatively, you can provide the precompiled libtcod library to override the
building process. [See below](#using-existing-binary-distribution).

To use `tcod-rs`, add this to your game's `Cargo.toml`:

```toml
[dependencies]
tcod = "*"
```

### Building on Linux

Run the equivalent of:

```sh
$ sudo apt-get install gcc g++ make libsdl1.2-dev
$ cd yourgame
$ cargo build
$ cargo run
```

on your distro.

You can also check the [official libtcod build instructions for Linux](http://roguecentral.org/doryen/data/libtcod/doc/1.5.2/html2/compile_libtcod_linux.html?c=true).


### Building on Windows (with MinGW)

The Windows version of `libtcod` relies on MinGW and MSYS so you have to install
them:

1. [Download and run MinGW](http://sourceforge.net/projects/mingw/files/)
2. In the MinGW installer, mark the following sections for installation:
   * C compiler (gcc)
   * C++ compiler (g++)
   * MSYS Basic System
3. Open the Command prompt (cmd.exe)
4. Run:

```sh
cd yourgame
cargo build
cargo run
```

You can also check the [official libtcod build instructions for Windows](http://roguecentral.org/doryen/data/libtcod/doc/1.5.2/html2/compile_libtcod_mingw.html?c=true).


### Building on Mac OS X

1. [Install Homebrew](http://brew.sh/)
2. Run:

```sh
$ brew install sdl wget
$ cd yourgame
$ cargo build
$ cargo run
```

This is based on the instructions from [Jared McFarland's roguelike tutorial](http://jaredonline.svbtle.com/roguelike-tutorial-in-rust-part-1).

---

To test this, you can clone this repository directly and run the one of the
provided examples:

```sh
$ git clone https://github.com/tomassedovic/tcod-rs.git
$ cd tcod-rs
$ cargo run --example keyboard
```


### Using existing binary distribution

If you don't want to build libtcod yourself, you can
[instruct Cargo to override the build script](http://doc.crates.io/build-script.html#overriding-build-scripts). See `.cargo/config`
from the repository for an example.

NOTE: The official MinGW pre-built libraries (for Windows)
[don't seem to work with tcod-rs](https://github.com/tomassedovic/tcod-rs/issues/54).
We're not sure exactly why this is so we'd appreciate anyone's help!


Contributing
------------

The raw bindings were generated using
[rust-bindgen](https://github.com/crabtw/rust-bindgen) and are located at
`src/ffi.rs`. The safe (hopefully?) wrapper was built on top of them at
`src/lib.rs`.

This is far from done, patches to missing functionality wrappers, documentation
and examples are very much appreciated. If your patch (any patch -- including
typos) gets accepted, you'll get a commit access if you want it.

We accept GitHub as well as regular pull requests (i.e. emailing or tweeting the
URL of your feature branch works).

You can regenerate the raw bindings by running:

```sh
bindgen -builtins -l tcod include/libtcod.h -o src/ffi.rs
```


### Contributors


* Bastien Léonard, @bastienleonard, <bastien.leonard@gmail.com>
* Edu Garcia, @Arcnor, <arcnorj@gmail.com>
* Guillermo Galizzi <galizzi.guillermo@gmail.com>
* Gustorn <gustorn@gmail.com>
* Jared McFarland, @jaredonline, <jared.online@gmail.com>
* Jonny Gilchrist, @jgilchrist
* LaylConway <laylconway@live.com>
* Moredread <code@andre-bubel.de>
* Nikita Pekin <contact@nikitapek.in>
* Paul Sanford, @pmsanford, <me@paulsanford.net>
* Pranz, <jesper.fridefors@gmail.com>
* Tomas Sedovic, @tomassedovic, <tomas@sedovic.cz>


### License

**tcod-rs** is licensed under [WTFPL v2](http://www.wtfpl.net/txt/copying/). See
`COPYING.txt` for the full text of the license (don't worry -- it's really
short and to the point).

# Warning: Not Maintained

This project is no longer actively developed or maintained. Please accept our apologies.

Open pull requests may still get merged, but we don't expect to investigate or fix open issues.

## Alternatives

There are two great alternatives to check out! They are both written in pure Rust (fewer chances of library linking or dependency issues, easier to deploy) and target the web browsers as well.

* [bracket-lib](https://github.com/thebracket/bracket-lib)
  * Written by [Herbert "TheBracket"](https://github.com/thebracket), author of [Nox Futura](https://thebracket.itch.io/nox-futura)
  * Comes with a [comprehensive tutorial](https://bfnightly.bracketproductions.com/rustbook/)
* [doryen-rs](https://github.com/jice-nospam/doryen-rs)
  * Written by [Jice](https://github.com/jice-nospam), the original author of libtcod (which this project provides bindings for)
  * Spiritual successor to libtcod

libtcod bindings for Rust [![Build Status](https://travis-ci.org/tomassedovic/tcod-rs.svg?branch=master)](https://travis-ci.org/tomassedovic/tcod-rs)
-------------------------

[libtcod a.k.a. "The Doryen Library"](https://github.com/libtcod/libtcod)
is a smallish library designed for writing roguelikes. It provides a bunch of
useful functionality such as:

* Text-based graphics API that doesn't suck as much as Curses or OpenGL
* Colours! (like, more than 16)
* Keyboard and mouse input
* Path finding
* Field of view
* Portable (works on linux, windows and mac)
* [Lots of other stuff](https://bitbucket.org/libtcod/libtcod/wiki/Features)


This project provides [Rust](http://www.rust-lang.org/) bindings for libtcod
v1.16.0-alpha.12.

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

https://libtcod.readthedocs.io/en/stable/


Current status
--------------

All raw tcod bindings are available via the `tcod-sys` crate. In addition, we
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
* _Pseudorandom number generator_ (prefer the `rand` crate, except for places where the API requires the built-in generators)
* _Name generator_
* _Image toolkit_
* _Line toolkit_
* _Noise_
* _BSP toolkit_

### Probably Won't Ever Be Implemented Because Rust Provides This Already
* Filesystem utilities
* Containers
* Compression toolkit (there will probably be a better Rust library for this)

### Not Implemented Yet But Should Happen At Some Point In The Future
* Everything else!



How to use this
---------------

`tcod-rs` depends on `libtcod` so you need to build or download the official
version. The `libtcod` version known to work comes bundled with `tcod-sys` and
Cargo will build it for you, but you need the build dependencies installed.

Alternatively, you can provide the precompiled libtcod library to override the
building process. [See below](#using-existing-binary-distribution).

To use `tcod-rs`, add this to your game's `Cargo.toml`:

```toml
[dependencies]
tcod = "0.16"
```

### Building on Linux

Run the equivalent of:

```sh
$ sudo apt-get install gcc g++ make libsdl2-dev
$ cd yourgame
$ cargo build --release
$ cargo run --release
```

on your distro.

#### Building a dynamic library

By default, `tcod-rs` will build the library statically on Linux as including
the code into the executable is usually more convenient. To build a dynamic
library specify the `dynlib` feature for `tcod-sys` in `Cargo.toml`

```
[dependencies.tcod-sys]
version = "*"
features = ["dynlib"]
```

### Building on Windows (with MSVC)

Make sure you have Visual Studio 2013 or later **with the C++ tools
option** installed. You also need the "MSVC ABI" version of the Rust
compiler (as opposed to the "GNU ABI" one).

Then, set up the compilation environment, make sure Rust is in your
`PATH` and run Cargo:

```
C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat amd64
set PATH=%PATH%;C:\Program Files (x86)\Rust\bin
cd yourgame
cargo build --release
cargo run --release
```


### Building on Windows (with MinGW)

You have to [download and install MinGW](http://www.mingw.org/). Then,
add Rust's and MinGW's bin directories to your path and compile your
game:

```
set PATH=%PATH%;C:\Program Files (x86)\Rust\bin;C:\MinGW\bin
cd yourgame
cargo build --release
cargo run --release
```


### Building on Mac OS X

1. [Install Homebrew](http://brew.sh/)
2. Run:

```sh
$ brew install pkg-config sdl2
$ cd yourgame
$ cargo build --release
$ cargo run --release
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

This is far from done, patches to missing functionality wrappers, documentation
and examples are very much appreciated. If your patch (any patch -- including
typos) gets accepted, you'll get a commit access if you want it.

We accept GitHub as well as regular pull requests (i.e. emailing or tweeting the
URL of your feature branch works).


### Contributors


* Alexander Krivács Schrøder <alexschrod@gmail.com>
* Bastien Léonard, @bastienleonard, <bastien.leonard@gmail.com>
* Darren Kaste, @dkaste, <darrenkaste@gmail.com>
* Edu Garcia, @Arcnor, <arcnorj@gmail.com>
* Guillermo Galizzi <galizzi.guillermo@gmail.com>
* Gustorn <gustorn@gmail.com>
* Jared McFarland, @jaredonline, <jared.online@gmail.com>
* Jonny Gilchrist, @jgilchrist
* LaylConway <laylconway@live.com>
* Moredread <code@andre-bubel.de>
* Nikita Pekin
* Paul Sanford, @pmsanford, <me@paulsanford.net>
* Pranz, <jesper.fridefors@gmail.com>
* Tomas Sedovic, @tomassedovic, <tomas@sedovic.cz>
* Tomasz Barański, <yubizume@gmail.com>


### License

**tcod-rs** is licensed under [WTFPL v2](http://www.wtfpl.net/txt/copying/). See
`COPYING.txt` for the full text of the license (don't worry -- it's really
short and to the point).

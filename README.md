libtcod bindings for Rust
-------------------------

[libtcod a.k.a. "The Doryen Library"](http://roguecentral.org/doryen/libtcod/)
is a smallish library designed for writing roguelikes. It provides a bunch of
useful functionality such as:

* Text-based graphics API that doesn't suck as much as curses or OpenGL
* Colours! (like, more than 16)
* Keyboard and mouse input
* Path finding
* Field of view
* Portable (works on linux and windows; mac too but requires some effort)
* [lots of other stuff](http://roguecentral.org/doryen/libtcod/features/)


This project provides [Rust](http://www.rust-lang.org/) bindings for libtcod
v1.5.1 (current stable).

**The only version tested and known to work is `libtcod 1.5.1`.** Link to other
versions (e.g. the `1.5.2` or `1.6.0` nightlies) at your own peril!

Rust is a new-ish systems programming language that aims to be fast (on the C++
level), memory and data-race safe, modern (first-class modules instead of a
preprocessor text-include hack, type inference, pattern matching, closures).

We track the nightly releases of Rust, usually lagging a few days behind. The
latest version of rustc tested:

    $ rustc --version
    rustc 0.11.0-pre-nightly (7ec7805 2014-06-16 08:16:49 +0000)

This project follows [Semantic Versioning](http://semver.org/). Since we're
under `1.0.0` anything goes. The API can change at any time.

Indeed, it probably should change! If you have better ideas on how it make it
safer or more familiar to Rust developers, please let us know.


Current status
--------------

All raw tcod bindings are available via the `ffi` module. In addition we want to
provide safe (and more in line with the Rust style) wrappers. These are far from
complete however.

### Implemented

* Most of the _Console_ features (new, init_root, blit, putting text on the
  screen, reading key presses, etc.)
* Most of _Map_ (new, size, set, is_walkable)
* A* and Dijkstra _Path_ finding
* Some of the _System_ layer (get/set FPS, last frame length)

### Probably Won't Ever Be Implemented Because Rust Provides This Already
* Filesystem utilities
* All purposes container
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

    [dependencies.tcod]
    git = "https://github.com/tomassedovic/tcod-rs.git"


### Building on Linux

Run the equivalent of:

    $ sudo apt-get install gcc g++ make upx electric-fence libsdl1.2-dev mercurial
    $ cd yourgame
    $ cargo build
    $ cargo run

on your distro.

You can also check the [official libtcod build instructions for Linux][linux].

linux: http://roguecentral.org/doryen/data/libtcod/doc/1.5.2/html2/compile_libtcod_linux.html?c=true


### Building on Windows (with MinGW)

The Windows version of `libtcod` relies on MinGW and MSYS so you have to install
them:

1. [Download and run MinGW][mingw]
2. In the MinGW installer, mark the following sections for installation:
   * C compiler (gcc)
   * C++ compiler (g++)
   * MSYS Basic System
3. Open the Command prompt (cmd.exe)
4. Run:

    cd yourgame
    cargo build
    cargo run

mingw: http://sourceforge.net/projects/mingw/files/

You can also check the [official libtcod build instructions for Windows][windows].

windows: http://roguecentral.org/doryen/data/libtcod/doc/1.5.2/html2/compile_libtcod_mingw.html?c=true


### Building on Mac OS X

1. [Install Homebrew](http://brew.sh/)
2. Run:

    $ brew install sdl wget
    $ cd yourgame
    $ cargo build
    $ cargo run

This is based on the instructions from [Jared McFarland's roguelike tutorial][macosx].

macosx: http://jaredonline.svbtle.com/roguelike-tutorial-in-rust-part-1

---

To test this, you can clone this repository directly and run the one of the
provided examples:

    $ git clone https://github.com/tomassedovic/tcod-rs.git
    $ cd tcod-rs
    $ cargo run --example keyboard


### Using existing binary distribution

If you don't want to build libtcod yourself, you can
[instruct Cargo to override the build script][override]. See `.cargo/config`
from the repository for an example.

override: http://doc.crates.io/build-script.html#overriding-build-scripts


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

Please make sure it builds against the latest `rustc`.

You can regenerate the raw bindings by running:

    bindgen -builtins -l tcod include/libtcod.h -o src/ffi.rs


### Contributors


* Bastien Léonard, @bastienleonard, <bastien.leonard@gmail.com>
* Edu Garcia, @Arcnor, <arcnorj@gmail.com>
* @Moredread
* Jared McFarland, @jaredonline, <jared.online@gmail.com>
* Paul Sanford, @pmsanford, <me@paulsanford.net>
* @Pranz, <jesper.fridefors@gmail.com>
* Tomas Sedovic, @tomassedovic, <tomas@sedovic.cz>


### License

**tcod-rs** is licensed under [WTFPL v2](http://www.wtfpl.net/txt/copying/). See
`COPYING.txt` for the full text of the license (don't worry -- it's really
short and to the point).

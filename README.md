libtcod bindings for Rust
-------------------------

[libtcod a.k.a. "The Doryen Library"](http://doryen.eptalys.net/libtcod/) is a
smallish library designed for writing roguelikes. It provides a bunch of useful
functionality such as:

* Text-based graphics API that doesn't suck as much as curses or OpenGL
* Colours! (like, more than 16)
* Keyboard and mouse input
* Path finding
* Field of view
* Kind of portable (a.k.a. what's this mac thing again?)
* [lots of other stuff](http://doryen.eptalys.net/libtcod/features/)


This project provides [Rust](http://www.rust-lang.org/) bindings for libtcod
v1.5.1 (current stable).

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
* Most of the A* _Path_ features (no Dijkstra yet)
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

_We'll provide code samples and a guide on building and linking this at some
point, but these don't exist yet, sorry!_

You can build this library by cloning the repository and typing `make`, link
your Rust binary against this and tcod dll/so which you can get from
[libtcod download page](http://doryen.eptalys.net/libtcod/download/).

Other than that, look at the source and read
[libtcod documentation](http://doryen.eptalys.net/data/libtcod/doc/1.5.1/index2.html?c=true&cpp=true&cs=true&py=true&lua=true).

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

* Tomas Sedovic <tomas@sedovic.cz>


### License

**tcod-rs** is licensed under [WTFPL v2](http://www.wtfpl.net/txt/copying/). See
`COPYING.txt` for the full text of the license (don't worry -- it's really
short and to the point).

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
* Portable (works on linux and windows; mac too but requires some effort)
* [lots of other stuff](http://doryen.eptalys.net/libtcod/features/)


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

You can build this library by cloning the repository and typing `cargo build`. You need to have `libtcod` in your library path, you can
download it from the [libtcod download page](http://doryen.eptalys.net/libtcod/download/), or install it using some package manager. See below for Mac OS X specific install instructions.

This will compile and link the library and a simple example in `target`

Other than that, look at the source and read
[libtcod documentation](http://doryen.eptalys.net/data/libtcod/doc/1.5.1/index2.html?c=true&cpp=true&cs=true&py=true&lua=true).

## OS X specific instructions

There are two dependencies to installing on OS X:
 * [Homebrew](http://brew.sh/)
 * `mercurial`
 * `sdl`
 * `wget`
 * `libtcod`

Unfortunately, `libtcod` is no longer officially supported on Mac OS X. As of 8/20/2014 the following instructinos work:

Start off by installing `mercurial`, `sdl` and `wget`:

```sh
brew install mercurial sdl wget
```

Then clone the offical `libtcod` repository (it helps if you put it in a safe place, like `~/src` or something):

```sh
hg clone https://bitbucket.org/jice/libtcod
```

Then make `libtcod` with the following commands:

 ```sh
cd libtcod
wget https://gist.githubusercontent.com/jaredonline/daf3c5f1ea6c7ca00e29/raw/ae91b3e47bf0de5b772eff882e477d8144cfbaf8/makefile-osx > makefiles/makefile-osx
wget https://dl.dropboxusercontent.com/u/169446/osx.tar.gz
tar -xzvf osx.tar.gz
make -f makefiles/makefile-osx
```

If this seems a bit convoluted that's because it is. I managed to find instructions [here](http://zackhovatter.com/gamedev/2013/11/26/building-libtcod-on-os-x-mavericks.html) but at the time of writing that link was broken. I grabbed a screenshot from Google Cache and put together my own packages of OS X specific materials. In step two above you're getting an OS X specific makefile and in step three you're getting some OS X specific headers.

You can test that it all worked by running

```sh
./samples_c
./samples_cpp
```

And you should get some pretty windows.

### Building a project with it:

Now, to get a project up and running with Cargo, add this to your `Cargo.toml` file:

```toml
build = "sh .build.sh"

[dependencies.tcod]
git = "https://github.com/tomassedovic/tcod-rs.git"
```

And add this file to your project root (`.build.sh`):

```sh
#!/bin/sh

export LIBTCOD_SRC_DIR="/PATH/TO/YOUR/src/libtcod"
cp $LIBTCOD_SRC_DIR/*.dylib $OUT_DIR/
cp $LIBTCOD_SRC_DIR/terminal.png $OUT_DIR/../../../
```

That copies the required `.dyblib` files from where you built `libtcod` to your project's target directory, and the `terminal.png` that `sdl` requires to your projects root directory.

After that you should be good to go! Happy building!

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

* Edu Garcia <arcnorj@gmail.com>
* Jared McFarland <jared.online@gmail.com>
* Tomas Sedovic <tomas@sedovic.cz>


### License

**tcod-rs** is licensed under [WTFPL v2](http://www.wtfpl.net/txt/copying/). See
`COPYING.txt` for the full text of the license (don't worry -- it's really
short and to the point).

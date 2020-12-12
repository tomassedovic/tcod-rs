#!/usr/bin/env bash

wget -O - https://www.libsdl.org/release/SDL2-2.0.12.tar.gz | tar xz
(cd SDL2-* && ./configure --prefix="$HOME/.local" && make -j 3 install)
export PATH=~/.local/bin:$PATH
export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$HOME/.local/lib/pkgconfig"

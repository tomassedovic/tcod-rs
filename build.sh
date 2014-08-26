#!/bin/sh

if [ -f lib/libtcod.so -o -f lib/libtcod.dylib ]; then
    cp lib/* "$OUT_DIR"
fi

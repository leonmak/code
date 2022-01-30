#!/bin/bash

mkdir -p target;

FILENAME=$1;
FN_NO_EXT="{$FILENAME%.rs}"
rustc $FILENAME -o target/$FN_NO_EXT && ./target/$FN_NO_EXT;

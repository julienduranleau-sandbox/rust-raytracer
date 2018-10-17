#!/bin/sh
rm out.bmp
cargo run
ppmtobmp out.ppm >> out.bmp
rm out.ppm

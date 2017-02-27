# What even is this?
Originally, this was a library to decode jpeg frames from an obscure
HDMI-over-LAN device. However, in the course of developing it, I've hit a weird
bug. In `src/bin/main.rs` in function `test_libs()`, if one uncomments lines
118 through 125, the function will fail much sooner.

The compiler used is `rustc 1.16.0-nightly (c07a6ae77 2017-01-17)`

# Dependencies
One should have a `libjpeg` compatible library installed on their machine for
this to compile successfully.

To test if the C code compiles, you can run 
```bash
gcc src/deco.c -c -ljpeg -fPIC -o decoc.o
```

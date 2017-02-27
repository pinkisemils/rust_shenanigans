# What even is this?
A crate to do weird things with an HDMI-over-IP device.
It also shows that at the time of writing, rust's jpeg library is 
about 4x slower than libjpeg-turbo.
The compiler used is `rustc 1.16.0-nightly (c07a6ae77 2017-01-17)`

# Dependencies
One should have a `libjpeg` compatible library installed on their machine for
this to compile successfully.

To test if the C code compiles, you can run 
```bash
gcc src/deco.c -c -ljpeg -fPIC -o decoc.o
```

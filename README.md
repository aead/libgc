# libgc
Rust library for garbled circuits - in cooperation with TU-Darmstadt and Niklas Büscher

For more information about the libgc format take a look at the [wiki page](https://github.com/aead/libgc/wiki/libgc-format).

What can be done with libgc?
 - Convert the output of the cbmc-gc compiler to the (smaller and more flexible) libgc format. (gc-convert)
 - Execute the binary circuit - even if the circuit consists of sub-circuits. (gc-binexec)

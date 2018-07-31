# kmod
## A Linux kernel module written in Rust

Dependency are the current Linux headers for your running kernel. To build the module simply execute in the root
directory:

```
make
```

To build a release execute:

```
make RELEASE=1
```

To cross-compile, specify `RUST_TARGET` and `KERNEL_BUILD_DIR`:

```
make RUST_TARGET=i686-unknown-linux-gnu KERNEL_BUILD_DIR=/path/to/linux-kernel-x.y.z/build
```

The kernel module can be found in `./target/kernel/kmod.ko`.


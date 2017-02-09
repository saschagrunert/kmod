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

The kernel module can be found in `./target/kernel/kmod.ko`.


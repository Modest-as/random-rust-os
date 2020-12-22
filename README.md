# Random Rust OS

* Get xbuild: `cargo install cargo-xbuild`

* Build: `cargo xbuild --target x86_64-margo_os.json`

* We need a nightly build: `rustup override add nightly`

* `xbuild` cross compiles Rust libraries so we need src: `rustup component add rust-src`

* We need a tool to link bootloader to our custom kernel: `cargo install bootimage --version "^0.5.0"`

* Build and link with bootloader: `bootimage build`

* Test: `bootimage run` or `sudo kvm ./target/x86_64-margo_os/debug/bootimage-margo_os.bin`

* Flash: `dd if=target/x86_64-margo_os/debug/bootimage-margo_os.bin of=/dev/sdX && sync`

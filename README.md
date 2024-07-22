# rust_os following by : https://os.phil-opp.com/
# run 
```bash
rustup override set nightly
cargo install bootimage
cargo bootimage
cargo build
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-rust_os.bin

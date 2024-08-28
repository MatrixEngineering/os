Simple os following source : https://os.phil-opp.com/
- install rust: https://www.rust-lang.org/tools/install
- install QEMU 
  - Ubuntu: 
     ```bash 
     $ apt-get install qemu-system
     ```
  - Mac OS:
     ```bash
     $ brew install qemu
     ```
  - Window: https://www.qemu.org/download/#windows
- install bootimage
```bash
$ cargo install bootimage
```
- set rust to nightly
```bash
$ rustup override set nightly
```
- run 
```bash
$ cargo run

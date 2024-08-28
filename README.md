Fun project simple os following source : https://os.phil-opp.com/
- make sure have install rust
- install QEMU 
     - Ubuntu: 
```bash 
$ apt-get install qemu-system
```
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

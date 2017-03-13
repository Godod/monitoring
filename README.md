# Monitoring tool
It's a terminal monitoring tool to monitor CPU, RAM, HDD

## Tested environment
1. Linux
a. Archlinux

## Install
0. [Install](https://www.rust-lang.org/en-US/install.html) rust lang:
```bash
curl https://sh.rustup.rs -sSf | sh
```

1. Clone the repo:
```rust
git clone https://github.com/Godod/monitoring.git
```

2. Build the application:
```bash
cd monitoring/
cargo build --release
```

3. Program path
```bash
target/release/monitoring
```

## Dependencies
1. [hashindexed](https://github.com/dhardy/hashindexed) = "0.1"
2. [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) = "0.2"
3. [regex](https://github.com/rust-lang/regex) = "0.2"
4. [term-painter](https://github.com/LukasKalbertodt/term-painter) = "0.2"
5. [term](https://github.com/Stebalien/term) = "0.4"

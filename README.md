# LauchOS
GPOS developed in Rust

Guideline: https://os.phil-opp.com https://os.phil-opp.com/edition-1/

VM: https://www.qemu.org/

Rust Documentation: https://doc.rust-lang.org/book/

# First time on dev device
1) Install rust-src  `rustup component add rust-src`
2) Install Bootimage  `cargo install bootimage`
3) Install llvm-tools  `rustup component add llvm-tools-preview`

# Start Program:
1) Build  `cargo build`
2) Run  `cargo run`

# Test Kernel (Qemu required):
1) Create Bootimage  `cargo bootimage`
2) Start Kernel  `qemu-system-x86_64 -drive format=raw,file=target/x86_64-lauch-os/debug/bootimage-lauch_os.bin`

# LauchOS
GPOS developed in Rust

Guideline: https://os.phil-opp.com https://os.phil-opp.com/edition-1/

VM: https://www.qemu.org/

Rust Documentation: https://doc.rust-lang.org/book/

# Requirements:
- Rust [Get Rust](https://www.rust-lang.org/learn/get-started)
- Qemu [Get Qemu](https://www.qemu.org/)

# First time on dev device
1) Install rust-src  `rustup component add rust-src`
2) Install Bootimage  `cargo install bootimage`
3) Install llvm-tools  `rustup component add llvm-tools-preview`
4) Install qemu

# Start Program (Qemu required):
1) Build  `cargo build`
2) Run  `cargo run`

# Start Kernel (Other option):
1) Create Bootimage  `cargo bootimage`
2) Start Kernel  `qemu-system-x86_64 -drive format=raw,file=target/x86_64-lauch-os/debug/bootimage-lauch_os.bin`

# Put it on USB-Stick:
1) sdX is your USB-Stick `sudo dd if=target/x86_64-lauch-os/debug/bootimage-lauch_os.bin of=/dev/sdX && sync`
!!IMPORTANT!! Be sure it is the right device

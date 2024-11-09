# Building from source
## Main kernel (on x86_64 Linux)
**Requirements:**
 - [Git](https://git-scm.com) version control system
 - [Rust toolchain](https://www.rust-lang.org/tools/install)

**Steps:**
 1. Install the nightly toolchain:
```bash
rustup toolchain install nightly
```
 2. Install required components:
```bash
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu && rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-gnu && cargo install bootimage
```
 3. Create a local clone of the repository:
```bash
git clone git@github.com:adamperkowski/highlightos.git && cd highlightos
```
 4. `cd` into the `kernel/` directory:
```bash
cd kernel
```
 5. Build the bootable binary:
```bash
cargo bootimage --release
```
This command will create a `target/target/release` directory in which you'll find the `bootimage-hlkernel.bin` binary file.

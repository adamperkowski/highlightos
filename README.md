# HighlightOS

<!-- logo instead of name -->

x86_64 OS (kernel) made completely from scratch using Assembly & Rust

[![GitHub Release](https://img.shields.io/github/v/release/adamperkowski/highlightos?label=Latest%20Released%20Version)](https://github.com/adamperkowski/highlightos/releases)

[![GitHub License](https://img.shields.io/github/license/adamperkowski/highlightos?label=License)](https://github.com/adamperkowski/highlightos/blob/main/LICENSE) ![GitHub repo size](https://img.shields.io/github/repo-size/adamperkowski/highlightos?label=Repo%20Size)

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/highlightos/asm.yml?branch=main&label=ASM%20Build)](https://github.com/adamperkowski/highlightos/actions) [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/highlightos/rust.yml?branch=main&label=HLShell%20Build)](https://github.com/adamperkowski/highlightos/actions)

Build instructions are available [here](#building-from-source-on-linux).

### It's not recommended to use any prebuilt binary files from the code section of this repository.

<!-- showcase -->
<!-- features -->
List of commands and features will be available soon.

<!-- installation & docs -->
# Building from source on Linux
Clone the HightlightOS GitHub repository into a directory of your choice. The examples below assume you have a machine capable of running HighlightOS or an emulator / VM like [QEMU](https://www.qemu.org).

## HLShell (the main kernel)
**Requirements:**
 - [Git](https://git-scm.com) version control system
 - [Rust toolchain](https://www.rust-lang.org/tools/install)

**Steps:**
 1. Install the nightly toolchain:<br>`rustup toolchain install nightly`
 2. Install required components:<br>`rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu && rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-gnu && cargo install bootimage`
 3. Clone the repository locally:<br>`git clone git@github.com:adamperkowski/highlightos.git && cd highlightos`
 4. CD into the HLS directory:<br>`cd shell`
 5. Build the bootable binary:<br>`cargo +nightly bootimage --release`<br><br>This command will create a `target/target/release` directory with the `bootimage-hlshell.bin` binary in it.

## HighlightOS ASM
**Requirements:**
 - [Git](https://git-scm.com) version control system
 - [NASM](https://nasm.us) Assembly compiler

**Steps:**
 1. Clone the repository locally:<br>`git clone git@github.com:adamperkowski/highlightos.git && cd highlightos`
 2. CD into the ASM directory:<br>`cd asm`
 3. Compile the bootable binary:<br>`nasm -f bin boot.asm -o boot.bin`<br><br>This command will create a `boot.bin` file in current directory.

 # Running in QEMU on Linux
 
 **Requirements:**
  - [QEMU](https://www.qemu.org/download/#linux) (full package)
  - A bootable binary of HighlightOS. You can download one through [releases](https://github.com/adamperkowski/highlightos/releases) or [build it yourself](#building-from-source-on-linux).

**Steps:**
 1. CD into directory containing the binary.
 2. Run:<br>`qemu-system-x86_64 -drive format=raw,file=<your_binary_filename>.bin`<br>Insert the name of downloaded / compiled binary into `<your_binary_filename>`.<br>
 3. Done!

<!-- contributing -->

<br><br>
*Copyleft 🄯 2024  Adam Perkowski*

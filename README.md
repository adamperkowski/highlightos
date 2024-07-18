# HighlightOS
### THIS main BRANCH CONTAINS CODE IN EARLY DEV STAGE. IF YOU WISH TO TRY OUT THE "WORKING" ONE, I CREATED A TEMPORARY BRANCH [exec](https://github.com/adamperkowski/highlightos/tree/exec).

<!-- logo instead of name -->

x86_64 OS (kernel) made completely from scratch using Assembly & Rust

[![GitHub Release](https://img.shields.io/github/v/release/adamperkowski/highlightos?label=Latest%20Released%20Version)](https://github.com/adamperkowski/highlightos/releases)

[![GitHub License](https://img.shields.io/github/license/adamperkowski/highlightos?label=License)](https://github.com/adamperkowski/highlightos/blob/main/LICENSE) ![GitHub repo size](https://img.shields.io/github/repo-size/adamperkowski/highlightos?label=Repo%20Size)

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/highlightos/asm.yml?branch=main&label=ASM%20Build)](https://github.com/adamperkowski/highlightos/actions) [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/highlightos/rust.yml?branch=main&label=HLShell%20Build)](https://github.com/adamperkowski/highlightos/actions)

Build instructions are available [here](#building-from-source-on-linux).<br>
Here are booting instructions for [QEMU](#running-in-qemu-on-linux) and [Real Hardware](#running-on-real-hardware).<br>
List of commands and features will be available soon.<br>
Contributing instructions will be available soon as well.

### It's not recommended to use any prebuilt binary files from the code section of this repository.

<!-- showcase -->
<!-- features -->
<!-- List of commands and features will be available soon. -->

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

# Running on real hardware
It's also possible to write the binary to a USB stick and boot on a real machine. You can write it by running:<br>
`dd if=<your_binary_filename>.bin of=/dev/sdX && sync`

Make sure to change `<your_binary_filename>.bin` to your downloaded / compiled binary name and **make sure to change `/dev/sdX` to correct device name of your USB because `dd` is a dangerous command and will overwrite any data it mets!**

After writing the image to a USB stick, you can run it on real hardware by just simply booting it.<br>Your BIOS menu should let you chose a device from which you want to boot.<br>**Note that HighlightOS won't work for UEFI-only machines, since it doesn't support UEFI yet.**

<!-- contributing -->

<br><br>
*Some parts of the code are inspired by [blog_os](https://github.com/phil-opp/blog_os). Great project!*

*Copyleft 🄯 2024  Adam Perkowski*

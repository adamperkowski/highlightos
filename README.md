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
<!-- installation & docs -->
# Building from source on Linux
Clone the HightlightOS GitHub repository into a directory of your choice. The examples below assume you have a machine capable of running HighlightOS or an emulator / VM like [QEMU](https://www.qemu.org).

## HighlightOS ASM
**Requirements:**
 - [Git](https://git-scm.com) version control system
 - [NASM](https://nasm.us) Assembly compiler

**Steps:**
 1. Clone the repository locally:<br>`git clone git@github.com:adamperkowski/highlightos.git && cd highlightos`
 2. CD into the ASM directory:<br>`cd asm`
 3. Compile the bootable binary:<br>`nasm -f bin boot.asm -o boot.bin`<br><br>This command will create a `boot.bin` file in current directory. 

<!-- contributing -->

<br><br>
Copyleft 🄯 2024  Adam Perkowski
# HighlightOS

<!-- logo instead of name -->

Un sistema operativo x86_64 (kernel) realizzato in Assembly e Rust.

[![README in English](https://img.shields.io/badge/Lang-%F0%9F%87%AC%F0%9F%87%A7-blue?color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/blob/main/README.md)
[![README in Polish](https://img.shields.io/badge/Lang-%F0%9F%87%B5%F0%9F%87%B1-blue?color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/blob/main/README-pl.md)
[![README in Italian](https://img.shields.io/badge/Lang-%F0%9F%87%AE%F0%9F%87%B9-blue?color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/blob/main/README-it.md)

[![GitHub Release](https://img.shields.io/github/v/release/adamperkowski/highlightos?label=Latest%20Released%20Version&color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/releases)
[![GitHub License](https://img.shields.io/github/license/adamperkowski/highlightos?label=License&color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/blob/main/LICENSE) ![GitHub repo size](https://img.shields.io/github/repo-size/adamperkowski/highlightos?label=Repo%20Size&color=%23ffcc4d&labelColor=%23000000)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/highlightos/asm.yml?branch=main&label=ASM%20Build&color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/actions) [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/highlightos/rust.yml?branch=main&label=HLShell%20Build&color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/actions)

## Table of contents
- [Compilare da sorgente su Mac/Linux](#compilare-da-sorgente-su-maclinux)
- [Eseguire in QEMU su Mac/Linux](#eseguire-in-qemu-su-maclinux)
- [Eseguire su hardware fisico](#eseguire-su-hardware-fisico)
- [Appendice](#appendice)
  - [Stato dello sviluppo](#stato-dello-sviluppo)
  - [Altre info](#altre-info)
  - [Crediti](#crediti)

<!-- showcase -->
<!-- features -->
<!-- List of commands and features will be available soon. -->

<!-- installation & docs -->
## Compilare da sorgente su Mac/Linux
### HLShell (kernel principale)
**Requisiti:**
 - [Git](https://git-scm.com) (controllo versione)
 - [Toolchain Rust](https://www.rust-lang.org/tools/install)

**Step:**
 1. Installa la toolchain nightly:
```bash
rustup toolchain install nightly
```
 2. Installa i componenti necessari:
```bash
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu && rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-gnu && cargo install bootimage
```
 3. Crea un clone locale di questa repository:
```bash
git clone git@github.com:adamperkowski/highlightos.git && cd highlightos
```
 4. `cd` dentro la cartella `shell/`:
```bash
cd shell
```
 5. Compila il file binario avviabile:
```bash
cargo +nightly bootimage --release
```
> Questo comando creerà la sottocartella `target/target/release` in cui verrà creato il file binario `bootimage-hlshell.bin`.

### HighlightOS ASM
**Requisiti:**
 - [Git](https://git-scm.com)
 - [NASM](https://nasm.us) compilatore Assembly

**Step:**
 1. Crea un clone locale di questa repository:
```bash
git clone git@github.com:adamperkowski/highlightos.git && cd highlightos
```
 2. `cd` nella cartella `asm/`:
```bash
cd asm
```
 3. Compila il file binario avviabile:
```bash
nasm -f bin boot.asm -o boot.bin
```
> Questo comando creerà il file binario `boot.bin` nella directory corrente.

## Eseguire in QEMU su Mac/Linux

**Requisiti:**
 - [QEMU](https://www.qemu.org/download/#linux) (pacchetto intero)
 - Un file binario avviabile di HighlightOS che puoi prendere da [releases](https://github.com/adamperkowski/highlightos/releases) o [compilarlo per conto tuo](#compilare-da-sorgente-su-maclinux).

**Step:**
 1. `cd` nella cartella che contiene il file binario.
 2. Esegui questo comando:
```bash
qemu-system-x86_64 -drive format=raw,file=<your_binary_filename>.bin
```
> [!IMPORTANT]
> Sostituisci `<your_binary_filename>` con il nome del file binario che hai scaricato/compilato.

## Eseguire su hardware fisico
Puoi anche flashare il file binario su una chiavetta USB e avviarlo da un computer fisico. 

Flasha il binario con questo comando:
```bash
dd if=<your_binary_filename>.bin of=/dev/sdX && sync
```

> [!IMPORTANT]
> Assicurati di sostituire `<your_binary_filename>.bin` con il nome del file binario e sostituisci `/dev/sdX` con la partizione della tua chiavetta USB. **Tutti i dati che contiene saranno cancellati!**

> [!NOTE]
> Puoi scegliere il dispositivo da cui avviare il computer mediante il boot menu del BIOS (accessibile premendo <kbd>F8</kbd> or <kbd>F12</kbd>).
>
>**<ins>Controlla che la tua scheda madre supporti il boot da media legacy</ins>, dato che HighlightOS non è (ancora) compatibile con UEFI.**

<!-- contributing -->
## Appendice

### Stato dello sviluppo
https://github.com/user-attachments/assets/f07cad18-845f-4457-a72a-a9bb4d3f6074

### Altre info
[Qui](https://github.com/adamperkowski/highlightos/wiki/Commands#built-in-commands) puoi trovare una lista di tutti i comandi disponibili.

Per scoprire di più, ti invitiamo a consultare la [wiki](https://github.com/adamperkowski/highlightos/wiki/).

**_È altamente sconsigliato l'uso di file binari precompilati provenienti dalla sezione "codice" di questa repo._**

### Contributori
**Un grazie speciale a tutti coloro che hanno scelto di contribuire al progetto:**

<a href="https://github.com/adamperkowski/highlightos/graphs/contributors">
  <img src="https://raw.githubusercontent.com/adamperkowski/highlightos/gh-pages/CONTRIBUTORS.svg"/>
</a>

### Crediti
*Alcune parti del codice sono ispirate da [blog_os](https://github.com/phil-opp/blog_os). Progetto fighissimo!*

*Traduzione in italiano a cura di/Italian translation by: [>franzageek<](https://github.com/franzageek)*<br>
*Copyleft 2024 Adam Perkowski*

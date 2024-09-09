# HighlightOS

<!-- logo zamiast nazwy -->

x86_64 System Operacyjny (kernel) zrobiony od zera w Assembly & Rust

[![README in English](https://img.shields.io/badge/EN-%F0%9F%87%AC%F0%9F%87%A7-blue?color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/blob/main/README.md)
[![README in Polish](https://img.shields.io/badge/PL-%F0%9F%87%B5%F0%9F%87%B1-blue?color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/blob/main/README-pl.md)
[![README in Italian](https://img.shields.io/badge/IT-%F0%9F%87%AE%F0%9F%87%B9-blue?color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/blob/main/README-it.md)

[![GitHub Release](https://img.shields.io/github/v/release/adamperkowski/highlightos?label=Latest%20Released%20Version&color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/releases)
[![GitHub License](https://img.shields.io/github/license/adamperkowski/highlightos?label=License&color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/blob/main/LICENSE) ![GitHub repo size](https://img.shields.io/github/repo-size/adamperkowski/highlightos?label=Repo%20Size&color=%23ffcc4d&labelColor=%23000000)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/highlightos/asm.yml?branch=main&label=ASM%20Build&color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/actions) [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/highlightos/rust.yml?branch=main&label=HLKernel%20Build&color=%23ffcc4d&labelColor=%23000000)](https://github.com/adamperkowski/highlightos/actions)

## Spis treści
- [Budowa ze źródła na Linuxie](#budowa-ze-źródła-na-linuxie)
- [Uruchamianie w QEMU na Linuxie](#uruchamianie-w-qemu-na-linuxie)
- [Uruchamianie na prawdziwym sprzęcie](#uruchamianie-na-prawdziwym-sprzęcie)
- [Załączniki](#załączniki)
  - [Status rozwoju](#status-rozwoju)
  - [Dodatkowe informacje](#dodatkowe-informacje)
  - [Przypisy](#przypisy)

<!-- pokaz -->
<!-- funkcję -->
<!-- Lista wszystkich komend i funkcji będą dostępne wkrótce. -->

<!-- instalacja & dokumenty -->
## Budowa ze źródła na Linuxie
### Główny Kernel
**Wymagania:**
 - System kontroli wersji[Git] (https://git-scm.com)
 - [Rust toolchain](https://www.rust-lang.org/tools/install)

**Kroki:**
 1. Zainstaluj toolchain nightly:
```bash
rustup toolchain install nightly
```
 2. Zainstaluj potrzebne komponenty:
```bash
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu && rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-gnu && cargo install bootimage
```
 3. Stwórz lokalną kopię repozytorium:
```bash
git clone git@github.com:adamperkowski/highlightos.git && cd highlightos
```
 4. Użyj `cd` do folderu `kernel/` :
```bash
cd kernel
```
 5. Zbuduj bootowalny plik binarny :
```bash
cargo +nightly bootimage --release
```
> Ta komenda stworzy folder `target/target/release` w którym znajdziesz `bootimage-hlkernel.bin`, czyli plik binarny.

### HighlightOS ASM
**Wymagania:**
 - System kontroli wersji [Git](https://git-scm.com)
 - Kompilator Assembly [NASM](https://nasm.us)

**Kroki:**
 1. Stwórz lokalną kopię repozytorium:
```bash
git clone git@github.com:adamperkowski/highlightos.git && cd highlightos
```
 2. Użyj `cd` do folderu `asm/` :
```bash
cd asm
```
 3. Skompiluj plik binarny:
```bash
nasm -f bin boot.asm -o boot.bin
```
> Ta komenda stworzy plik `boot.bin` i umieści go w twoim aktualnym folderze.

## Uruchamianie w QEMU na Linuxie

**Wymagania:**
 - [QEMU](https://www.qemu.org/download/#linux) (cały pakiet)
 - Bootwalny plik binarny HighlightOS. Możesz takowy pobrać z [wydań](https://github.com/adamperkowski/highlightos/releases), albo [zbudować własny](#Budowa-z-źródła-na-Linuxie).

**Kroki:**
 1. Użyj `cd` do folderu z plikiem binarnym.
 2. Użyj następującej komendy:
```bash
qemu-system-x86_64 -drive format=raw,file=<your_binary_filename>.bin
```
> [!IMPORTANT]
> Upewnij się że zamieniłeś `<nazwa_twojego_pliku_binarnego>` na właściwą nazwę pliku który pobrałeś/zbudowałeś.

## Uruchamianie na prawdziwym sprzęcie
Możesz również sflaszować plik binarny do swojego USB pedrive'a i uruchomić go na prawdziwej maszynie

Żeby to zrobić wykorzystaj następującą komende:
```bash
dd if=<your_binary_filename>.bin of=/dev/sdX && sync
```

> [!IMPORTANT] 
> Upewnij się że zamieniłeś `<nazwa_twojego_pliku_binarnego>` na właściwą nazwę pliku który pobrałeś/zbudowałeś i upewnij się żeby zamienić `/dev/sdX` na numerem partycji z twojego USB. **Wszelkie dane zostaną starocone!**

> [!NOTE]
>Możesz wybrać urządzenie bootujące z poziomu BIOS boot menu (dostępne poprzez klikanie <kbd>F8</kbd> lub <kbd>F12</kbd>).
>
>**<ins>Sprawdź czy twoja płyta główna obsługuję bootowanie z starszych (legacy) nośników</ins>, gdyż HighlightOS nie jest jeszcze kompatybilny z UEFI.**

<!-- contributing -->
## Załączniki

### Status rozwoju
https://github.com/user-attachments/assets/de796522-3d6b-4672-a335-ff282422fe86

### Dodatkowe informacje
**Czy wiedziałeś że mamy kanał IRC? Jest to '#highlightos' na [libera.chat](https://libera.chat).**

Lista wbudowanych komend i funkcji jest dostępna [tutaj](https://github.com/adamperkowski/highlightos/wiki/Commands#built-in-commands).<br>
Po więcej informacji na temat HighlightOS, zapraszamy na nasze [wiki](https://github.com/adamperkowski/highlightos/wiki/).

**_Używanie prekompilowanych plików binarnych z sekcji kodu tego repo nie jest zalecane._**

### Współtwórcy
**WIELKIE podziękowania każdemu współtwórcy:**

<a href="https://github.com/adamperkowski/highlightos/graphs/contributors">
  <img src="https://raw.githubusercontent.com/adamperkowski/highlightos/gh-pages/CONTRIBUTORS.svg"/>
</a>

### Przypisy
*Niektóre części kodu były inspirowane [blog_os](https://github.com/phil-opp/blog_os). Świetny projekt!*

*Polskie tłumaczenie: [Sebaguardian](https://github.com/Sebaguardian)*<br>
*Copyleft 2024 Adam Perkowski*

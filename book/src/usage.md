# Usage
## Running in QEMU on Linux
**Requirements:**
 - [QEMU](https://www.qemu.org/download/#linux) (full package)
 - A bootable binary of HighlightOS<br>You can download one from [releases](https://github.com/adamperkowski/highlightos/releases) or you can [build it yourself](./building.html).

**Steps:**
 1. `cd` into directory that contains the binary.
 2. Run the following command:
```
qemu-system-x86_64 -drive format=raw,file=<your_binary_filename>.bin
```
Replace `<your_binary_filename>` with the actual name of the HighlightOS binary.

## Running on real hardware
You can also flash the binary image onto a USB stick and boot it on a real machine. 

You can flash it by running the following command:
```
dd if=<your_binary_filename>.bin of=/dev/sdX && sync
```

Make sure to replace `<your_binary_filename>.bin` with your downloaded/compiled binary name and make sure to replace `/dev/sdX` with your USB's actual partition number.<br>**Any data on it will be lost!**

You can choose the device to boot off of from your BIOS boot menu (accessible by pressing <kbd>F2</kbd>, <kbd>F8</kbd> or <kbd>F12</kbd> - depending on your motherboard).

**<ins>Double-check that your motherboard is capable of booting legacy media(s)</ins>, as HighlightOS is not UEFI-compatible yet.**

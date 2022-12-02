# Kleos

A rudimentary operating system built solely in Rust for 
learning and experimentation purposes.

## TODO:

- [x] Booting with BIOS
  + using `bootloader v0.11`
- [x] Text output
  + [x] Serial port to stdout with `uart_16550`
  + [x] Framebuffer rendering
    * [x] Bitmap font ([`cozette`](https://github.com/slavfox/Cozette))
          with a from-scratch rendering implementation.
- [ ] Unit and Integration tests
- [x] Interrupts
  - [x] CPU Exceptions (Page Fault, Double Fault, etc)
  - [x] APIC Hardware Interrupts
- [ ] Memory management
  - [x] Access to physical memory at virtual location
  - [x] Access to paging and page allocation
  - [ ] Heap allocation
- [ ] Filesystem Drivers
  - [ ] *TODO:* What does this need?
- [ ] Abstract into dynamically loaded modules
  - [ ] *TODO:* What does this need?


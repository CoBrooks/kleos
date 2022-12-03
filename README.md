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
- [ ] Booting with UEFI
- [x] Interrupts
  - [x] CPU Exceptions (Page Fault, Double Fault, etc)
  - [x] APIC Hardware Interrupts
- [x] Memory management
  - [x] Access to physical memory at virtual location
  - [x] Access to paging and page allocation
  - [x] Heap allocation
    + using Fixed Size Block + Linked List Allocators
    + See [here](https://os.phil-opp.com/allocator-designs/#fixed-size-block-allocator).
- [ ] Concurrency with Rust `async` and `await`
- [ ] Filesystem Drivers
  - [ ] *TODO:* What does this need?
- [ ] Abstract into dynamically loaded modules
  - [ ] *TODO:* What does this need?


# Execution Order

1. Core Functionality
  - [X] Serial output
  - [X] GDT / IDT
  - [X] APIC
  - [X] Heap
  - [_] Tasks
  - [_] Syscalls

2. Tracing
  - [~] Capture traces with metadata

3. Drivers
  - [~] Framebuffer
  - [_] Keyboard

4. Usermode
  - [_] ???

# Project Structure

```
kernel/
|-- src/main.rs
|-- Cargo.toml
|-- kcore/
.   |-- Cargo.toml
.   |-- src/
.   .   |-- lib.rs
.   .   |-- interrupts.rs
.   .   |-- apic.rs
.   .   |-- gdt.rs
.   .   |-- serial.rs
.   .   |-- memory.rs
.   .   |-- allocator/
.   .   .   |-- mod.rs
.   .   .   |-- fixed_size_block.rs
|-- ktracing/
.   |-- Cargo.toml
.   |-- src/
.   .   |-- lib.rs
.   .   |-- tracing.rs
|-- drivers/
.   |-- Cargo.toml
.   |-- framebuffer/ (or display?)
.   .   |-- Cargo.toml
.   .   |-- src/
.   .   .   |-- lib.rs
.   .   .   |-- color.rs
.   .   .   |-- font.rs
.   |-- ps2/
.   .   |-- Cargo.toml
.   .   |-- src/
.   .   .   |-- lib.rs
.   .   .   |-- keyboard.rs
```

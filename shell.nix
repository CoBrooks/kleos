{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
  buildInputs = [
    qemu_kvm
  ];
}

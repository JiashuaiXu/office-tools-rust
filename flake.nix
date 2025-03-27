{
  description = "Rust nightly development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [
        pkgs.rust-bin.nightly.latest.default
        pkgs.cargo
        pkgs.rustfmt
        pkgs.clippy
        pkgs.rust-analyzer
        pkgs.cowsay
        pkgs.lolcat
      ];

      shellHook = ''
        echo "Welcome to the Rust nightly dev shell!" | lolcat
        cowsay "Rust version: $(rustc --version)" | lolcat
      '';
    };
  };
}


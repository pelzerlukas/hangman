{
  description = "hangman in rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          clippy
          rustfmt
          rust-analyzer
          pkg-config
        ];
        RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
      };

      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "my-rust-app";
        version = "0.1.0";
        src = ./.;

        cargoLock.lockFile = ./Cargo.lock;

        nativeBuildInputs = [ pkgs.pkg-config ];

        # If you build a binary and want a nicer `nix run`, add:
        # meta.mainProgram = "my-rust-app";
      };
    };
}

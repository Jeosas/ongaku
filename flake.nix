{
  description = "rust project template";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    fenix,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [fenix.overlays.default];
        };

        rust' = pkgs.fenix.latest; # Rust toolchain selection

        naersk' = pkgs.callPackage naersk {inherit (rust') cargo rustc;};

        dependencies = with pkgs; [yt-dlp ffmpeg];
        build_dependencies = with pkgs; [protobuf];
      in {
        packages = rec {
          default = ongaku;
          ongaku = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = [pkgs.makeWrapper];
            buildInputs = build_dependencies;
            postInstall = with pkgs; ''
              wrapProgram $out/bin/ongaku \
                --set PATH ${lib.makeBinPath dependencies}
            '';
          };
        };

        devShell = with pkgs;
          mkShell rec {
            packages =
              [
                # General
                just

                # Nix
                alejandra

                # Rust
                (rust'.withComponents [
                  "cargo"
                  "clippy"
                  "rust-src"
                  "rustc"
                  "rustfmt"
                ])
                rust-analyzer
                cargo-watch

                # Dev deps
                jq
              ]
              ++ dependencies
              ++ build_dependencies;

            LD_LIBRARY_PATH = lib.makeLibraryPath packages;
          };
      }
    );
}

# this is mostly from https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10#a-flake-with-derivation
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        # !! toolchain goes here
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        nativeBuildInputs = with pkgs; [pkg-config rustToolchain];
        buildInputs = with pkgs; [
          # !! needed libraries go here
          xdotool
          openssl
          cairo
          gdk-pixbuf
          pango
          libsoup_3
          gtkd
          atkmm
          webkitgtk_4_1
        ];
      in
        with pkgs; {
          devShells.default = mkShell {
            inherit buildInputs nativeBuildInputs;
            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
            packages = [dioxus-cli];
            shellHook = ''
              echo "entering pigcast devshell..."
            '';
          };
        }
    );
}

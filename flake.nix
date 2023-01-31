{
  description = "E2E encrypted chat";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = github:numtide/flake-utils;
  };
  outputs = {self, nixpkgs, flake-utils, ...}@inputs:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          nodejs-19_x
          cargo
          rustc
          gcc
          nodePackages.pnpm
        ];
        buildInputs = with pkgs; [
          rustfmt
          rust-analyzer
          dbus
          pkgconfig
          openssl
          sass
          glib
          cairo
          pango
          atk
          gdk-pixbuf
          libsoup
          gtk3
          webkitgtk
          librsvg
          patchelf
        ];
      };
    });
}
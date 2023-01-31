{
  description = "E2E encrypted chat";
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = github:numtide/flake-utils;
  };
  outputs = {self, nixpkgs, flake-utils, ...}@inputs:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      libraries = with pkgs; [
        webkitgtk
        gtk3
        cairo
        gdk-pixbuf
        glib
        dbus
        openssl_3
        librsvg
      ];
      packages = with pkgs; [
        curl
        wget
        pkg-config
        dbus
        openssl_3
        glib
        gtk3
        libsoup
        webkitgtk
        nodejs-19_x
        rust-analyzer
        rustfmt
        rustc
        gcc
        sass
        nodePackages.pnpm
        cargo
      ];
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = packages;
        shellHook = ''
          export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
        '';
      };
    });
}
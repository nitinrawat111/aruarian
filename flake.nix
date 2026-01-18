# Reference: https://v1.tauri.app/v1/guides/getting-started/prerequisites/#setting-up-linux
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        rustToolchain = fenix.packages.${system}.stable.withComponents [
          "rustc"
          "cargo"
          "rustfmt"
          "clippy"
          "rust-src"
        ];
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer

            # Node.js
            nodejs_24

            # Build tools
            pkg-config
            curl
            wget

            # Tauri deps
            dbus
            glib
            gtk3
            libsoup_3
            webkitgtk_4_1
            openssl
            librsvg
            gdk-pixbuf
            cairo
          ];

          shellHook = ''
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
            export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
          '';
        };
      }
    );
}

# Tauri V1 flake reference: https://v1.tauri.app/v1/guides/getting-started/prerequisites/#setting-up-linux
# Tauri V2 nix shell reference: https://wiki.nixos.org/wiki/Tauri
{
  description = "Tauri dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            cargo
            nodejs_24
            rustup
          ];

          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
          ];

          shellHook = ''
          export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
            pkgs.gtk3
            pkgs.gdk-pixbuf
            pkgs.glib
            pkgs.cairo
            pkgs.pango
            pkgs.webkitgtk_4_1
            pkgs.libsoup_3
          ]}
          rustup default stable
          echo "Tauri dev shell ready"
          '';
        };
      }
    );
}

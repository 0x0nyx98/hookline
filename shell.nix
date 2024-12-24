let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    #cargo
    #cargo-tauri
    nodejs
  ];

  buildInputs = with pkgs;[
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
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.at-spi2-atk}/lib:${pkgs.atkmm}/lib:${pkgs.cairo}/lib:${pkgs.gdk-pixbuf}/lib
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.glib}/lib:${pkgs.gtk3}/lib:${pkgs.harfbuzz}/lib:${pkgs.librsvg}/lib
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.libsoup_3}/lib:${pkgs.pango}/lib:${pkgs.webkitgtk_4_1}/lib:${pkgs.openssl}/lib:${pkgs.glib.out}/lib
  '';
}
with import <nixpkgs> {};
pkgs.mkShell {
  buildInputs = [
    pkg-config
    libarchive
    llvmPackages.libclang
  ];
  shellHook = ''
    export LIBCLANG_PATH="${llvmPackages.libclang}/lib"
  '';
}

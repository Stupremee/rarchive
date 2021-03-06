with import <nixpkgs> {};
pkgs.mkShell {
  buildInputs = [
    pkg-config
    libarchive
    llvmPackages.libclang
    openssl
    bzip2
    zlib
    lzma
    valgrind
  ];
  shellHook = ''
    export LIBCLANG_PATH="${llvmPackages.libclang}/lib"
  '';
}

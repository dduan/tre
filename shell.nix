with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "tre-env";
  nativeBuildInputs = [
    rustup
  ];
  RUST_BACKTRACE = 1;
}

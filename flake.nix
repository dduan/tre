{
  description = "Tree command, improved.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs.legacyPackages.${system};
      rec {
        packages = flake-utils.lib.flattenTree {
          tre-command = rustPlatform.buildRustPackage rec {
            pname = "tre-command";
            version = "0.3.6";
            src = ./.;
            cargoSha256 = "1f7yhnbgccqmz8hpc1xdv97j53far6d5p5gqvq6xxaqq9irf9bgj";
            lockFile = ./Cargo.lock;
            nativeBuildInputs = [ installShellFiles ];
            preFixup = ''
              installManPage manual/tre.1
            '';
          };
        };
        defaultPackage = packages.tre-command;
        devShell = pkgs.mkShell {
          buildInputs = [
            cargo
            rust-analyzer
            rustc
          ];
        };
      });
}

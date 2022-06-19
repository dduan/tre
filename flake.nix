{
  description = "Tree command, improved.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-21.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs.legacyPackages.${system};
      let info = (fromTOML (builtins.readFile ./Cargo.toml)).package; in
      rec {
        packages = flake-utils.lib.flattenTree {
          ${info.name} = rustPlatform.buildRustPackage rec {
            pname = info.name;
            version = info.version;
            src = ./.;
            cargoSha256 = "sha256-b3fScJMG/CIkMrahbELLQp1otmT5En+p8kQsip05SOc=";
            lockFile = ./Cargo.lock;
            nativeBuildInputs = [ installShellFiles ];
            preFixup = ''
              installManPage manual/tre.1
              installShellCompletion scripts/completion/tre.{bash,fish}
              installShellCompletion --zsh scripts/completion/_tre
            '';
            # this test requires package to be in a git repo to succeed
            checkFlags = "--skip respect_git_ignore";
          };
        };
        defaultPackage = packages.${info.name};
        devShell = pkgs.mkShell {
          buildInputs = [
            cargo
            clippy
            rust-analyzer
            rustc
          ] ++ pkgs.lib.lists.optionals stdenv.isDarwin [
            libiconv
          ];
        };
      });
}

{
  description = "Contact manager based on the KDBX4 encrypted database format";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }: (
    flake-utils.lib.eachDefaultSystem (
      system: (
        let
          projectName = "keep-in-touch";
          pkgs = import nixpkgs {
            inherit system;
          };

          cargoPackages = with pkgs; [
            cargo
            rustc
            rustfmt
          ];
        in {
          devShells = {
            default = pkgs.mkShell {
              buildInputs = cargoPackages;

              shellHook = ''
                export RUSTFLAGS='-C target-cpu=native'
              '';
            };
          };
          packages = {
            default = pkgs.rustPlatform.buildRustPackage rec {
              pname = projectName;
              version = "main";

              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
                outputHashes = {
                  # This hash need to be updated everytime you bump the version of the keepass-rs
                  # library.
                  "keepass-0.6.0-rc.1" = "sha256-C9oRCxFfRdny+5CmEf0VYaBT9gkL7+CXlnypeTT2Oxg=";
                };
              };

              meta = with pkgs.lib; {
                description = "Contact manager based on the KDBX4 encrypted database format";
                homepage = "https://github.com/louib/${projectName}";
                license = licenses.gpl3;
                # maintainers = [];
              };
            };
          };
        }
      )
    )
  );
}

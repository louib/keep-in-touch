{
  description = "CLI tool to translate from kdbx to vcard, and vice versa";

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
          projectName = "kp2vcard";
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
            };
          };
          packages = {
            kp2vcard = pkgs.rustPlatform.buildRustPackage rec {
              pname = projectName;
              version = "main";

              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              meta = with pkgs.lib; {
                description = "CLI tool to translate from kdbx to vcard, and vice versa";
                homepage = "https://github.com/louib/${projectName}";
                # license = licenses.unlicense;
                # maintainers = [maintainers.tailhook];
              };
            };
          };
        }
      )
    )
  );
}

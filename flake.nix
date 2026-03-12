{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let 
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = pkgsRaw: overlays: evaluation: (nixpkgs.lib.genAttrs supportedSystems) (system:  evaluation system (import pkgsRaw {
        inherit system;
        inherit overlays;
      }));
    in {
      overlays.default = final: prev: { hello = self.hello final; };
      packages = forAllSystems nixpkgs [] (system: pkgs: rec {
          default = glashaus;
          glashaus = self.glashaus pkgs;
          });
      devShells = forAllSystems nixpkgs [] (system: pkgs: {
        default = pkgs.mkShellNoCC rec {
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = with pkgs; [
            rustc
            cargo
          ];
        };
      });
      glashaus = pkgs: pkgs.rustPlatform.buildRustPackage rec {
        name = "glashaus";
        src = ./.;
        cargoLock.lockFile = "${src}/Cargo.lock";
        buildInputs = with pkgs; [
          cargo
        ];
      };
    };
}

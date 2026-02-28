{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let 
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      forAllSystemsTest = pkgsRaw: evaluation: (nixpkgs.lib.genAttrs supportedSystems) (system:  evaluation system pkgsRaw.${system});

      pkgsRaw = nixpkgs.legacyPackages;
      null = (val: { inherit val; none = ""; });
    in rec {

      packages = forAllSystemsTest pkgsRaw (system: pkgs:
      rec {
        default = pkgs.rustPlatform.buildRustPackage (finalAttrs: rec {
          pname = "glashaus";
          version = "0.0.1";
          src = ./.;
          cargoHash = "";
          cargoLock = {
            lockFile = "${src}/Cargo.lock";
          };
        });
      });

        #default = buildDerivation system pkgs {
        #  name = "glashaus";
        #  buildInputs = with pkgs; [
        #    cargo
        #    git
        #    coreutils
        #    findutils
        #  ];
        #  buildPhase = ''
        #  cargo build --release -v -j1
        #  '';
        #  installPhase = ''
        #  cp $src/target/release/$name $out/bin
        #  '';
        #  outputHashAlgo = "sha256";
        #  outputHashMode = "recursive"; # Specify "recursive" for directories, "flat" for files
        #  outputHash = ""; # Initially unknown, so leave it empty and let Nix find it
        #};

      buildDerivation = system: pkgs: attrs: 
      derivation (rec {
          buildInputs = [ pkgs.coreutils ];
          inherit system;
          builder = "${pkgs.bash}/bin/bash";
          preBuild = "export HOME=$TMP";
          args = [ "${(builderfile system pkgs)}"];
          src = ./.;
      } // attrs);

      builderfile = system: pkgs: 
      derivation rec {
          name = "builderfile";
          inherit system;
          builder = "${pkgs.bash}/bin/bash";
          preBuild = ''cd $src
          '';
          args = [ "-c" ''echo "
set -e
unset PATH
for p in \$buildInputs; do
    export PATH=\$p/bin:\$PATH
done
export TMP=\$(${pkgs.coreutils}/bin/mktemp -d)
${pkgs.coreutils}/bin/mkdir -p \$out/bin
echo "[NIXBUILDER] preBuild"
eval \$preBuild
echo "[NIXBUILDER] buildPhase"
eval \$buildPhase
echo "[NIXBUILDER] installPhase"
eval \$installPhase
echo "[NIXBUILDER] cleanUpPhase"
eval \$cleanUpPhase
" > $out''];
      };
    };
}

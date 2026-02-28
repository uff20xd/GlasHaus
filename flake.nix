{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let 
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgs = nixpkgs.legacyPackages;
      dir = ./.;
      system_test = "x86_64-linux";
      null = (val: { inherit val; none = ""; });
    in {

      builder = system: {
        default = derivation {
          name = "buildertomycoolprogram";
          buildInputs = with pkgs.${system}; [
            gcc
            coreutils
            rustc
          ];
          src = "${dir}/build.rs";
          inherit system;
          builder = "${pkgs.${system}.bash}/bin/bash";
          args = [ "${dir}/nix/builder_builder.sh" ];
        };};

      some_package = system: {
        default = derivation {
          name = "buildertomycoolprogram";
          buildInputs = with pkgs.${system}; [
            gcc
            coreutils
            rustc
          ];
          src = "${(self.builder system).default}/build";
          inherit system;
          builder = "${pkgs.${system}.bash}/bin/bash";
          args = [ "${dir}/nix/build.sh" ];
        };};

      packages = forAllSystems (system: (self.some_package) system);
    };
}

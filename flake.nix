# ffp: FFP-ALATYRATAL-FLAKE declarative nix
{
  description = "AlatyrAtal — Proof-of-Quality-Adjusted-Inference Clearinghouse & Dual-Token Engine (Unit of Ndimmôrwyn)";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in {
      packages = forAllSystems (system:
        let pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "alatyratal";
            version = "1.0.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
          };
        });
    };
}

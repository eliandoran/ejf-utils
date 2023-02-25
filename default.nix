{ pkgs, ... }:

let
  pname = "ejf-utils";
in {
  app = pkgs.rustPlatform.buildRustPackage {
    pname = pname;
    version = "0.0.1";
    src = ./.;
    cargoBuildFlags = "-p ${pname}";
    cargoLock = {
      lockFile = ./Cargo.lock;
    };
    nativeBuildInputs = with pkgs; [ cmake ];
  };
}
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};

      buildPkgs = with pkgs; [
        pkg-config
        scdoc
      ];

      libPkgs = with pkgs; [
        openssl_3
      ];

      devPkgs = with pkgs; [
        cargo
        rustfmt
        rustc
        just
        cargo-tarpaulin
        vhs
      ];
    in {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "temporis";
        version = "1.0.0";
        src = ./.;
        cargoHash = "sha256-Li4pxu1JnIfuOGy51/FrFj5DTZ3oWuzg647qYgWyGmk=";

        nativeBuildInputs = buildPkgs;
        buildInputs = libPkgs;

        cargoBuildFlags = ["--lib"];
        doCheck = true;
        RUSTDOCFLAGS = "-D warnings";
        doDoc = true;
      };

      devShell = pkgs.mkShell {
        nativeBuildInputs = buildPkgs;
        buildInputs = libPkgs ++ devPkgs;
      };
    });
}

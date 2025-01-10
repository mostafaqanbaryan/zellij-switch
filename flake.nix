{
  inputs = {
    nixpkgs.url = "nixpkgs";
    systems.url = "github:nix-systems/default";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs @ { self, nixpkgs, systems, rust-overlay, ... }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = eachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-wasip1" ];
          };
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "zellij-switch";
            version = "0.2.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            
            nativeBuildInputs = [ rustToolchain ];

            buildPhase = ''
              cargo build --target wasm32-wasip1 --release
            '';

            installPhase = ''
              mkdir -p $out/bin
              cp target/wasm32-wasip1/release/*.wasm $out/bin/
            '';

            doCheck = false;
          };
        });

      defaultPackage = eachSystem (system:
        self.packages.${system}.default);

      overlays.default = final: prev: {
        zellij-switch = self.packages.${prev.system}.default;
      };
    };
}

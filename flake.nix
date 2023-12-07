{
  description = "いろんなwebhookを取りたいtraQ BOT(rust) 開発中";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-U2yfueFohJHjif7anmJB5vZbpP7G6bICH4ZsjtufRoU=";
        };
        rustPlatform = pkgs.makeRustPlatform {
          rustc = toolchain;
          cargo = toolchain;
        };
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [ openssl libiconv ]
          ++ lib.optionals stdenvNoCC.isDarwin [ darwin.Security darwin.apple_sdk.frameworks.SystemConfiguration ];
        defaultBuildArgs = {
          pname = "cnvtr";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          inherit nativeBuildInputs buildInputs;
          doCheck = false;
          buildType = "debug";
        };
        buildRustPackage = attrs: rustPlatform.buildRustPackage (defaultBuildArgs // attrs);
      in
      {
        devShells.default = pkgs.stdenv.mkDerivation {
          name = "bot-cnvtr";
          nativeBuildInputs = with pkgs; nativeBuildInputs ++ [ toolchain cargo-make sqlx-cli grcov ];
          inherit buildInputs;
        };
        packages = {
          default = buildRustPackage { };
          with-check = buildRustPackage {
            checkPhase = ''
              cargo fmt --all -- --check
              cargo clippy --all-targets --all-features -- -D warnings
              cargo test --no-run
            '';
            doCheck = true;
            installPhase = ''
              mkdir -p $out
              mv ./target/* $out/
            '';
          };
        };
      }
    );
}

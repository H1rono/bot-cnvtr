{
  description = "いろんなwebhookを取りたいtraQ BOT(rust) 開発中";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    teahook-rs = {
      url = "github:H1rono/teahook-rs";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        fenix.follows = "fenix";
        crane.follows = "crane";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, crane, teahook-rs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        inherit (pkgs) lib;

        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-SXRtAuO4IqNOQq+nLbrsDFbVk+3aVA8NNpSZsKlVH/8=";
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        octokit-webhooks = pkgs.fetchFromGitHub {
          owner = "octokit";
          repo = "webhooks";
          rev = "v7.3.1";
          hash = "sha256-ckGVw5owHTv1h73LGan6mn4PZls4sNjRo/n+rrJHqe0=";
        };

        gitea = pkgs.fetchFromGitHub {
          owner = "traptitech";
          repo = "gitea";
          rev = "traP-1.21.1-1";
          hash = "sha256-3iMenHuWaJFhMm7s5zoNuC/DebziVNhLWQrEsuiDNHM=";
        };
        teahook-transpiler = teahook-rs.packages.${system}.goBuild;

        commonArgs = {
          pname = "bot-cnvtr";
          inherit src;
          strictDeps = true;
          nativeBuildInputs = [ pkgs.pkg-config ];
          # Common arguments can be set here to avoid repeating them later
          buildInputs = with pkgs; [
            # Add additional build inputs here
            openssl
          ] ++ lib.optionals stdenvNoCC.isDarwin [
            # Additional darwin specific inputs can be set here
            libiconv
            darwin.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          # Additional environment variables can be set directly
          CARGO_PROFILE = "";
          GITHUB_WEBHOOK_SCHEMA_DTS = "${octokit-webhooks}/payload-types/schema.d.ts";
          GITEA_SOURCE_ROOT = "${gitea}";
          GITEA_TRANSPILER_PATH = "${teahook-transpiler}/bin/teahook-rs";
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        build = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit build;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          # Check formatting
          fmt = craneLib.cargoFmt commonArgs;
        };
        packages = {
          default = build;
          cargoDeps = cargoArtifacts;
          otherDeps = pkgs.symlinkJoin {
            name = "cnvtr-other-deps";
            paths = [ octokit-webhooks gitea teahook-transpiler ];
          };
        };

        apps.default = flake-utils.lib.mkApp {
          drv = build;
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";
          GITHUB_WEBHOOK_SCHEMA_DTS = "${octokit-webhooks}/payload-types/schema.d.ts";
          GITEA_SOURCE_ROOT = "${gitea}";
          GITEA_TRANSPILER_PATH = "${teahook-transpiler}/bin/teahook-rs";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [ ];
        };
      }
    );
}

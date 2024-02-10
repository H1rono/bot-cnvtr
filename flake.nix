{
  description = "いろんなwebhookを取りたいtraQ BOT(rust) 開発中";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane/v0.16.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    teahook-rs = {
      url = "github:H1rono/teahook-rs";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        fenix.follows = "fenix";
        crane.follows = "crane";
        gitea.follows = "gitea";
      };
    };
    octokit-webhooks = {
      url = "github:octokit/webhooks/v7.3.1";
      flake = false;
    };
    gitea = {
      url = "github:traPtitech/gitea/traP-1.21.1-1";
      flake = false;
    };
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , fenix
    , crane
    , teahook-rs
    , octokit-webhooks
    , gitea
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
      inherit (pkgs) lib dockerTools;

      toolchain = fenix.packages.${system}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-SXRtAuO4IqNOQq+nLbrsDFbVk+3aVA8NNpSZsKlVH/8=";
      };
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
      src = craneLib.cleanCargoSource (craneLib.path ./.);

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
      releaseArgs = commonArgs // {
        CARGO_PROFILE = "release";
      };

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      # Build the actual crate itself, reusing the dependency
      # artifacts from above.
      debugBuild = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
      });

      releaseArtifacts = craneLib.buildDepsOnly releaseArgs;
      releaseBuild = craneLib.buildPackage (releaseArgs // {
        cargoArtifacts = releaseArtifacts;
      });

      docker.bash = dockerTools.buildImage {
        name = "bash";
        tag = "latest";
        copyToRoot = pkgs.buildEnv {
          name = "image-root";
          paths = [ pkgs.bashInteractive ];
          pathsToLink = [ "/bin" ];
        };
      };
      docker.release = dockerTools.buildImage {
        name = "bot-cnvtr";
        tag = "latest";
        fromImage = docker.bash;
        copyToRoot = pkgs.buildEnv {
          name = "image-root";
          paths = [
            releaseBuild
            pkgs.openssl
            pkgs.coreutils
            dockerTools.caCertificates
          ];
        };
        config.Cmd = [ "/bin/bot-cnvtr" ];
      };
    in
    {
      checks = {
        # Build the crate as part of `nix flake check` for convenience
        inherit debugBuild;

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
        default = debugBuild;
        cargoDeps = cargoArtifacts;
        release = releaseBuild;
        cargoDepsRelease = releaseArtifacts;
        otherDeps = pkgs.symlinkJoin {
          name = "cnvtr-other-deps";
          paths = [ octokit-webhooks gitea teahook-transpiler ];
        };
        releaseImage = docker.release;
      };

      apps.default = flake-utils.lib.mkApp {
        drv = debugBuild;
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
    });
}

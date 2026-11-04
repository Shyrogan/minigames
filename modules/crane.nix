{inputs, ...}: {
  perSystem = {pkgs, ...}: let
    craneLib = inputs.crane.mkLib pkgs;

    commonArgs = {
      src = craneLib.cleanCargoSource ../.;
      strictDeps = true;
    };

    # Every dependency in Cargo.lock, built once and reused until the lockfile
    # changes.
    cargoArtifacts = craneLib.buildDepsOnly commonArgs;
  in {
    packages.deps = cargoArtifacts;

    checks = {
      clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });
      test = craneLib.cargoTest (commonArgs // {inherit cargoArtifacts;});
    };
  };
}

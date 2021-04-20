let
  unstableTgz = builtins.fetchTarball {
    # Descriptive name to make the store path easier to identify
    name = "nixos-unstable-2021-04-04";
    # Be sure to update the above if you update the archive
    url = https://github.com/nixos/nixpkgs-channels/archive/4762fba469e2baa82f983b262e2c06ac2fdaae67.tar.gz;
    sha256 = "1sidky93vc2bpnwb8avqlym1p70h2szhkfiam549377v9r5ld2r1";
  };
in
import unstableTgz {}

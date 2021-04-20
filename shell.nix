let
  unstable = import ./unstable.nix;
in
{ pkgs ? unstable  }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustChannels.stable.rust
  ];
}

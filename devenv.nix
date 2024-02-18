{pkgs, ...}: let
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in {
  name = "Wordlers";

  pre-commit.hooks = {
    cargo-check.enable = true;
    clippy.enable = true;
    commitizen.enable = true;
    rustfmt.enable = true;
  };

  packages = [toolchain];

  languages = {
    rust = {
      enable = true;
      toolchain = toolchain;
    };
  };
}

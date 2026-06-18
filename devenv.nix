{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  packages = [
    pkgs.pkg-config
    pkgs.python3

    pkgs.sqlx-cli
  ];

  languages.rust.enable = true;

}

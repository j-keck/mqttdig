{ pkgs ? import <nixpkgs> {} }:
let

  easy-ps = import (pkgs.fetchFromGitHub {
    owner = "justinwoo";
    repo = "easy-purescript-nix";
    rev = "aa94aeac3a6ad9b4dfa0e807ad1421097d74f663";
    sha256 = "1kfhi6rscgf165zg4f1s0fgppygisvc7dppxb93n02rypxfxjirm";
  }) { inherit pkgs; };


  buildInputs =
    (with pkgs; [ dhall nodejs ]) ++
    (with pkgs.nodePackages; [ parcel-bundler ]) ++
    (with easy-ps; [ purs spago spago2nix ]);

in pkgs.mkShell {
  inherit buildInputs;

  shellHooks = ''
    alias serv="parcel bundle --out-dir dist/ index.html";
    alias dist="parcel build --out-dir dist/ index.html";
  '';
}

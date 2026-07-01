{
  description = "Rust Devshell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      requiredPackages = with pkgs; [
        stdenv.cc
        rust-analyzer
        expat
        fontconfig
        freetype
        rustfmt
        freetype.dev
        libGL
        pkg-config
        libx11
        libxcursor
        libxi
        libxrandr
        wayland
        libxkbcommon
        rustc
        cargo
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell rec {
        packages = requiredPackages;
        buildInputs = requiredPackages;
        LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
      };
    };
}

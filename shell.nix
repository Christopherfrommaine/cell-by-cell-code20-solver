let
  pkgs = import <nixpkgs> {};
in pkgs.mkShell {
  buildInputs = [
    
    pkgs.rustup
    pkgs.imagemagick
    
  ];
  
  shellHook = ''
    export RUST_LOG=info
  '';
}

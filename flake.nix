{
  inputs = {
    nixpkgs.url = "github:xinux-org/nixpkgs/nixos-unstable";
    xinux-lib = {
      url = "github:xinux-org/lib";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat.url = "github:NixOS/flake-compat";
  };

  outputs = {self, ...} @ inputs:
    inputs.xinux-lib.mkFlake {
      inherit inputs;
      alias.packages.default = "nixos-conf-editor";
      alias.shells.default = "nixos-conf-editor";
      src = ./.;
      hydraJobs = {
        inherit (self.packages.x86_64-linux) default;
      };
    };
}

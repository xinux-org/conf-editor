{
  inputs = {
    nixpkgs.url = "github:xinux-org/nixpkgs/nixos-25.11";
    xinux-lib = {
      url = "github:xinux-org/lib/release-25.11";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
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

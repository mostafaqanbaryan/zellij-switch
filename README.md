# Zellij switch

I needed this for my workflow, but it's is not great at all and has a few problems.

But works for me!

## Instruction

    zellij pipe --plugin https://github.com/mostafaqanbaryan/zellij-switch/releases/download/0.2.1/zellij-switch.wasm -- "--session zellij-session --cwd /home --layout default"

- `-s|--session` as of `0.2.1`, can include spaces
- `-c|--cwd` must be an absolute path and is optional.
- `-l|--layout` is optional


## Build

- Clone the project and run `cargo build --target wasm32-wasip1 --release`

## Installation on NixOS

If you use home-manager or maintain a Nix configuration, you can add `zellij-switch` as easy as:

1. Add this flake as an input in your Nix configuration:

```nix
{
  inputs.zellij-switch.url = "github:mostafaqanbaryan/zellij-switch";
}
```

2. Add `zellij-switch` overlay to your `nixpkgs`:

```nix
{
  nixpkgs.overlays = [ zellij-switch.overlays.default ];
}
```

3. Add `zellij-switch` to your packages:

```nix
environment.systemPackages = with pkgs; [
  zellij-switch
];
```

4. Apply your configuration:

```bash
nixos-rebuild switch
```

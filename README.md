# Zellij switch

I needed this for my workflow, but it's is not great at all and has a few problems.

But works for me!

## Instruction

    zellij pipe --plugin https://github.com/mostafaqanbaryan/zellij-switch/releases/download/v0.2.0/zellij-switch.wasm -- "--session zellij-session --cwd /home --layout default"

- `-s|--session` cannot have any space (like any other zellij session name).
- `-c|--cwd` must be an absolute path and is optional.
- `-l|--layout` is optional


## Build

- Clone the project and run `cargo build --target wasm32-wasip1 --release`

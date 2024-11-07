# Zellij switch

I needed this for my workflow, but it's is not great at all and has a few problems.

But works for me!

## Instruction

    zellij pipe --plugin https://github.com/mostafaqanbaryan/zellij-switch/releases/download/v0.1.2/zellij-switch.wasm -- "$session_name::$cwd"

As of version 0.1.1, this works without specifying `$cwd`:

    zellij pipe --plugin https://github.com/mostafaqanbaryan/zellij-switch/releases/download/v0.1.2/zellij-switch.wasm -- "$session_name"

Starting at version 0.1.2, you may additionally specify a layout (without .kdl extension) in the third position:

    zellij pipe --plugin https://github.com/mostafaqanbaryan/zellij-switch/releases/download/v0.1.2/zellij-switch.wasm -- "$session_name::$cwd::$layout"

3. (Optional) For better integration, [use this script](https://github.com/mostafaqanbaryan/dotfiles/blob/main/scripts/sessions)

## Build

- Clone the project and run `cargo build --target wasm32-wasip1 --release`

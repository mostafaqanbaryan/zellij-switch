# Zellij switch
I needed this for my workflow, but it's is not great at all and has a few problems.

But works for me!

## Instruction
1. `zellij pipe --plugin https://github.com/mostafaqanbaryan/zellij-switch/releases/download/v0.1.1/zellij-switch.wasm -- "$session_name::$cwd"`
   - As of version 0.1.1, this works without specifying `$cwd`:
   - `zellij pipe --plugin https://github.com/mostafaqanbaryan/zellij-switch/releases/download/v0.1.1/zellij-switch.wasm -- "$session_name"`
3. (Optional) For better integration, [use this script](https://github.com/mostafaqanbaryan/dotfiles/blob/main/scripts/sessions)

## Build
- Clone the project and run `cargo build --target wasm32-wasi --release`

# Zellij switch
I needed this for my workflow, but it's is not great at all and has a few problems.

But works for me!

## Instruction
1. Clone the project and run `cargo build --target wasm32-wasi --release`
2. `zellij pipe --plugin https://github.com/mostafaqanbaryan/zellij-switch/releases/download/v0.1.0/zellij-switch.wasm -- "$session_name::$cwd"`
3. (Optional) For better integration, [use this script](https://github.com/mostafaqanbaryan/dotfiles/blob/main/scripts/sessions)

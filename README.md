# Wasm Tests 
I'm using this repo to test and debug possible ways of using rust + wasm as a scripting system in my game engine.

## How to run
Run `cargo run` inside of WasmtimeTests to run the test executable.

## Building wasm scripts
Run `compileScript.sh` in TestWasmScripts to compile testScript1 and testScript2 into separate wasm binaries. 

To use these then replace the wasm files in WasmtimeTests with the new ones you just built. (Likely found in `TestWasmScripts/target/wasm32-wasi/debug/`)

## End goal
The engine that I'm building is architected around an entity component system framework, so the workflow I'm trying to figure out is something like this:

    
Systems will be one or multiple script files compiled into one binary that must be able to access arrays of entities/components very efficiently
As I have a need to load and unload systems at runtime, they need to be able to share memory with each-other. 
I would also like to multi-thread ecs api calls, so I both need to be able to share memory between separate system binaries, but also between multiple instances of those binaries on separate threads.

Longer term it would be nice to be able to import/export several different wasm memories depending on security needs. For example, having a private stack memory for modules, but shared memory for sections of ecs memory. (Or one shared memory for the module and it's threaded instances, that's not shared with other modules)

This is where I think about going back to my scripting language project, since I'm not expecting rust to implement multi-memory support soon.

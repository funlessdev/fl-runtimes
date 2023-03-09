# The builder of FunLess Rust functions

This project consists of a Dockerfile that mounts a FunLess Rust function as a volume 
and takes care of compiling it into a wasm module via `cargo wasi build`, ensuring the compatibility with the Wasm runtime in the backend.
The Wasm produced should accept a JSON input and return a JSON output.

The `lib_fl` folder contains an example Rust project, respecting the current required constraints for Rust functions (by using the `fl-wasm-rs` crate).


Current constraints:

- The crate must be of type `lib`
- The crate must define a function called `fl_main`, with type `serde_json::Value -> serde_json::Value` (the function can panic, as stderr is captured in the backend).

# Running the docker image

The docker image requires two volumes:

- The user's function, which needs to be mounted as `/lib_fl` inside the container
- An output directory, which needs to be mounted as `/out_wasm` inside the container

To produce the `code.wasm` for the example function inside the directory `out_wasm`, the run command would be:

```
docker run -v $(pwd)/lib_fl:/lib_fl -v $(pwd)/out_wasm:/out_wasm <IMAGE_NAME>
```
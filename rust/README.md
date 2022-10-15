# Funless wrapper for Rust functions

This project contains a small wrapper for Rust functions, ensuring the compatibility with the Wasm runtime in the backend.
The final Wasm produced receives a JSON object via stdin and returns a JSON object via stdout.

The `lib_fl` folder contains an example Rust project, respecting the current required constraints for Rust functions.


Current constraints:

- The crate must be of type `lib`
- The crate must export one function called `fl_main`, with type `serde_json::Value -> serde_json::Value` (the function can panic, as stderr is captured in the backend)



# Running the docker image

The docker image requires two volumes:

- The user's function, which needs to be mounted as `/lib_fl` inside the container
- An output directory, which needs to be mounted as `/out_wasm` inside the container

To produce the `code.wasm` for the example function inside the directory `out_wasm`, the run command would be:

```
docker run -v $(pwd)/lib_fl:/lib_fl -v $(pwd)/out_wasm:/out_wasm <IMAGE_NAME>
```
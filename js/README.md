# Funless wrapper for Javascript functions

This project contains a small wrapper for Javascript functions, ensuring the compatibility with the Wasm runtime in the backend.
The final Wasm produced receives a JSON string via stdin and returns a JSON string via stdout.

The `lib_fl` folder contains an example Javascript project, respecting the current required constraints for Javascript functions.


Current constraints:

- The code must be a valid js library
- The library must export a `fl_main` function (e.g. `module.exports.fl_main = fl_main`)




# Running the docker image

The docker image requires two volumes:

- The user's function, which needs to be mounted as `/lib_fl` inside the container
- An output directory, which needs to be mounted as `/out_wasm` inside the container

To produce the `code.wasm` for the example function inside the directory `out_wasm`, the run command would be:

```
docker run -v $(pwd)/lib_fl:/lib_fl/ -v $(pwd)/out_wasm:/out_wasm <IMAGE_NAME>
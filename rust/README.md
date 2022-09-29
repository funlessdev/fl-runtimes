# Funless wrapper for Rust functions

This project contains a small wrapper for Rust functions, ensuring the compatibility with the WASM runtime in the backend.

The `lib_fl` folder contains an example Rust project, respecting the current required constraints for Rust functions.


Current constraints:

- The crate must be of type `lib`
- The crate must export one function called `fl_main`, with type `serde_json::Value -> serde_json::Value` (the function can panic, as stderr is captured in the backend)
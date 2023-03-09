// Copyright 2022 Giuseppe De Palma, Matteo Trentin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use fl_wasm_rs::prelude::*;
use serde::Deserialize;

#[derive(Default, FLFunction)]
struct MyFn;

#[derive(Default, Deserialize)]
struct Person {
    name: String,
}

fn fl_main(input: serde_json::Value) -> FLResult {
    let parsed_input = serde_json::from_value::<Person>(input);
    match parsed_input {
        Ok(person) => {
            let out = format!("Hello {}!", person.name);
            let json: Result<serde_json::Value, serde_json::Error> =
                serde_json::from_str(&format!(r#"{{"payload": "{}" }}"#, &out));
            json.map_err(|_| FLError::ExecError("Failed to parse JSON".to_string()))
        }
        Err(e) => Err(FLError::ExecError(format!("Failed to parse input: {}", e))),
    }
}

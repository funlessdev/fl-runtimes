// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.
//

fn main() {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Rust Wrapper: couldn't read input");

    let json: serde_json::Value =
        serde_json::from_str(&input).expect("Rust Wrapper: input JSON not well-formatted");
    let output = lib_fl::fl_main(json);

    let _result = serde_json::to_writer(std::io::stdout(), &output)
        .expect("Rust Wrapper: couldn't format output to json");
}

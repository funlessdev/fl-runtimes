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

pub fn fl_main(body: serde_json::Value) -> serde_json::Value {
    let parsed_body: String = serde_json::from_value(body).expect("Failed to parse JSON in input");
    let out = format!("Hello {}!", parsed_body);
    serde_json::from_str(&format!(r#""{}""#, &out)).expect("Failed to parse JSON in project")
}

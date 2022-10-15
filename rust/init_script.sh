# Copyright 2022 Giuseppe De Palma, Matteo Trentin
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

#!/bin/sh
mkdir /proj/lib_fl
cp -r /lib_fl/* /proj/lib_fl/
echo "$(cat /proj/lib_fl/Cargo.toml | dasel put string -r toml 'package.name' 'lib_fl')" > /proj/lib_fl/Cargo.toml
cargo wasi build --release
mv /proj/target/wasm32-wasi/release/wrapper.wasm /out_wasm/code.wasm
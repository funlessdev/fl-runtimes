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
FROM rust:alpine

RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache curl


RUN curl -sSLf "$(curl -sSLf https://api.github.com/repos/tomwright/dasel/releases/latest | grep browser_download_url | grep linux_amd64 | grep -v .gz | cut -d\" -f 4)" -L -o dasel && \
    chmod +x dasel && \
    mv ./dasel /bin/dasel

RUN rustup target add wasm32-wasi
RUN cargo install cargo-wasi

COPY init_script.sh .

WORKDIR /proj

COPY . .
RUN rm -r ./lib_fl/*

VOLUME ["/lib_fl/"]
VOLUME ["/out_wasm"]

# copy to destination folder (another mount) with name "code.wasm"
CMD ["sh", "/init_script.sh"]




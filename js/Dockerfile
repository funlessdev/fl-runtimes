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
FROM rust:slim as builder

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y cmake git make bash curl clang musl-dev gcc g++

RUN rustup target add wasm32-wasi
RUN git clone https://github.com/Shopify/javy.git && \
    cd /javy && \
    make download-wasi-sdk && \
    make

##########

FROM node:current-slim

COPY --from=builder /javy/target/release/javy /usr/bin/javy

RUN apt-get update && \
    apt-get upgrade -y

RUN npm install -g parcel

COPY init_script.sh .

WORKDIR /proj

COPY . .
RUN rm -r ./lib_fl/*

VOLUME ["/lib_fl/"]
VOLUME ["/out_wasm"]


CMD ["sh", "/init_script.sh"]
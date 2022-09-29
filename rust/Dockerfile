# dependencies:
# cargo; cargo wasi; curl; dasel; rustup (we need to add wasm32-wasi target); possibly gcc/g++/binutils/build-base
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




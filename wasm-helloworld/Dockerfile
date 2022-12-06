FROM scratch
COPY ./target/wasm32-wasi/release/wasm-helloworld.wasm /wasm-helloworld.wasm
ENTRYPOINT [ "wasm-helloworld.wasm" ]
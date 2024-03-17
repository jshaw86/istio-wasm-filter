FROM rust:1.76-buster AS wasm-builder
WORKDIR /build
COPY . /build
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release

FROM scratch
COPY --from=wasm-builder /build/target/wasm32-wasi/release/istio_http_filter.wasm ./plugin.wasm


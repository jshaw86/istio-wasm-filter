FROM rust:1.76-buster AS wasm-builder
WORKDIR /build
COPY . /build
RUN cargo install wasm-pack
RUN wasm-pack -v build

FROM scratch
COPY --from=wasm-builder /build/pkg/istio_http_filter_bg.wasm ./plugin.wasm

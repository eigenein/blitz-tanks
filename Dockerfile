FROM arm64v8/rust:1.69 AS compiler
WORKDIR /usr/src/blitz-tanks
COPY . .
RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install --path=. --locked

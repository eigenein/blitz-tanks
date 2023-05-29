FROM arm64v8/rust:1.69 AS compiler
WORKDIR /usr/src/blitz-tanks
COPY . .
RUN cargo install --path=. --locked

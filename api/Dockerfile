## Builder
FROM rustlang/rust:nightly-slim as cargo-build

# Build dependencies
WORKDIR /usr/src/api
RUN mkdir src \
    && echo "fn main() {println!(\"Dummy main\");}" > src/main.rs
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build
RUN rm src/*.rs
RUN rm -f target/debug/deps/api*

FROM rustlang/rust:nightly-slim as cargo-run

WORKDIR /usr/src/api

# Copy custom code
COPY --from=cargo-build /usr/src/api/target/debug /usr/src/api/target/debug
COPY . .
RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]

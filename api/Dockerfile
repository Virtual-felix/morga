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

# Build api
COPY . .
RUN cargo build

## Runner
FROM debian:buster-slim

ARG API_USER=api
RUN groupadd ${API_USER} \
    && useradd -g ${API_USER} ${API_USER} \
    && mkdir -p /home/${API_USER}

WORKDIR /home/${API_USER}
COPY --from=cargo-build /usr/src/api/target/debug/api /home/${API_USER}/
RUN chown -R ${API_USER}:${API_USER} ./api

USER ${API_USER}
CMD ["./api"]

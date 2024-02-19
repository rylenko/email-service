FROM rustlang/rust:nightly AS chef
RUN cargo +nightly install cargo-chef
WORKDIR /usr/src/

FROM chef AS planner
COPY . .
RUN cargo +nightly chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/recipe.json recipe.json
RUN cargo +nightly chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo +nightly build --release

FROM rustlang/rust:nightly AS runtime
WORKDIR /usr/src/
COPY . .
RUN cargo +nightly install diesel_cli --no-default-features --features postgres
COPY --from=builder /usr/src/target/release/launcher /usr/local/bin/
COPY ./client/entrypoint.sh ./client/entrypoint.sh
RUN chmod a+x ./client/entrypoint.sh
ENTRYPOINT ["./client/entrypoint.sh"]

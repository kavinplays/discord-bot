FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN rustup install nightly
RUN cargo +nightly build --release
RUN echo '/usr/local/lib' >> /etc/ld.so.conf
RUN cat /etc/ld.so.conf
RUN ldconfig
RUN echo 'export LD_LIBRARY_PATH=/usr/local/lib' >> ~/.bash_profile && . ~/.bash_profile


FROM debian:buster-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/discord-bot /usr/local/bin
ENTRYPOINT ["/usr/local/bin/discord-bot"]
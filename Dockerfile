FROM liuchong/rustup:nightly-musl-onbuild as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/gtrend-api /gtrend-api
COPY Rocket.toml /Rocket.toml

CMD ["/gtrend-api"]

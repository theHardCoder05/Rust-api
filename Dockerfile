
FROM rust as builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM rust as runtime
WORKDIR /app
COPY --from=builder /app/target/release/rest_api .
COPY --from=builder /app/Rocket.toml .
EXPOSE 8000
CMD ["./rest_api"]
FROM rust:1.90 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --target-dir target



FROM scratch

WORKDIR /app


COPY --from=builder /app/target/release/pdf2images_app /app/pdf2images_app

EXPOSE 5800

CMD ["/app/pdf2images_app"]
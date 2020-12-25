FROM rust:1.48-alpine3.11 as builder
WORKDIR /usr/src/app1
COPY . .
RUN cargo install --path .

FROM alpine:3.11
COPY --from=builder /usr/local/cargo/bin/app1 /usr/local/bin/app1
CMD ["app1"]
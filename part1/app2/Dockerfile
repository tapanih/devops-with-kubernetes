FROM ekidd/rust-musl-builder:nightly-2020-11-19 as builder
WORKDIR /usr/src/app2
COPY . .
RUN sudo chown -R rust:rust . && cargo install --path .

FROM alpine:3.12
COPY --from=builder /home/rust/.cargo/bin/app2 /usr/local/bin/app2
CMD ["app2"]
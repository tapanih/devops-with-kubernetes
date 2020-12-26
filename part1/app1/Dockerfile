FROM ekidd/rust-musl-builder:nightly-2020-11-19 as builder
WORKDIR /usr/src/app1
COPY . .
RUN sudo chown -R rust:rust . && cargo install --path .

FROM alpine:3.12
COPY --from=builder /home/rust/.cargo/bin/app1 /usr/local/bin/app1
CMD ["app1"]
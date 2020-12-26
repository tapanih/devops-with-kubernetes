FROM ekidd/rust-musl-builder:nightly-2020-11-19 as builder
WORKDIR /usr/src/app-response
COPY . .
RUN sudo chown -R rust:rust . && cargo install --path .

FROM alpine:3.12
COPY --from=builder /home/rust/.cargo/bin/app-response /usr/local/bin/app-response
RUN mkdir /usr/local/bin/files
RUN mkdir /usr/local/bin/pongs
ENV TIMESTAMP_FILE=/usr/local/bin/files/timestamp.txt
ENV PINGPONG_FILE=/usr/local/bin/pongs/pingpong.txt
CMD ["app-response"]
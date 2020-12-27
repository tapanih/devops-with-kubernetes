FROM ekidd/rust-musl-builder:nightly-2020-11-19 as builder
WORKDIR /usr/src/pingpong
COPY . .
RUN sudo chown -R rust:rust . && cargo install --path .

FROM alpine:3.12
COPY --from=builder /home/rust/.cargo/bin/pingpong /usr/local/bin/pingpong
RUN mkdir /usr/local/bin/pongs
ENV PINGPONG_FILE=/usr/local/bin/pongs/pingpong.txt
CMD ["pingpong"]
FROM ekidd/rust-musl-builder:nightly-2020-11-19 as builder
WORKDIR /usr/src/timestamper
COPY . .
RUN sudo chown -R rust:rust . && cargo install --path .

FROM alpine:3.12
COPY --from=builder /home/rust/.cargo/bin/timestamper /usr/local/bin/timestamper
RUN mkdir /usr/local/bin/files
ENV TIMESTAMP_FILE=/usr/local/bin/files/timestamp.txt
CMD ["timestamper"]
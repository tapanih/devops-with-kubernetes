FROM ekidd/rust-musl-builder:nightly-2020-11-19 as build
WORKDIR /usr/src/todo-service
COPY . .
RUN sudo chown -R rust:rust . && cargo install --path .

FROM alpine:3.12
COPY --from=build /home/rust/.cargo/bin/todo-service .
ENV REDIRECT_URL=http://localhost:8081/
CMD ["./todo-service"]

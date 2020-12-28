FROM ekidd/rust-musl-builder:nightly-2020-11-19 as build
WORKDIR /usr/src/todo-app
COPY . .
RUN sudo chown -R rust:rust . && cargo install --path .

FROM alpine:3.12
COPY --from=build /home/rust/.cargo/bin/todo-app .
RUN mkdir templates && mkdir images
COPY templates ./templates
ENV IMAGE_FILE=/images/daily.jpeg
ENV TODO_SERVICE_URL=http://todo-service-svc:2348/todos
ENV TODO_POST_URL=http://localhost:8081/todos
CMD ["./todo-app"]

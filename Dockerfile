FROM gcr.io/distroless/cc-debian12:latest AS prod

WORKDIR /workspace

ADD target/x86_64-unknown-linux-gnu/release/helloworld-server /workspace/helloworld-server

CMD ["/workspace/helloworld-server"]

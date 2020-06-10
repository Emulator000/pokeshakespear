FROM alpine:latest

RUN apk --update upgrade && \
    apk add curl ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/cache/apk/*

COPY ./config/config.toml /home/config/config.toml
COPY ./target/x86_64-unknown-linux-musl/release/pokeshakespear /home/pokeshakespear

RUN chmod a+x /home/pokeshakespear

WORKDIR /home

CMD /home/pokeshakespear

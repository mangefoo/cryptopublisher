FROM debian:bullseye-slim

RUN apt update && apt install -y ca-certificates libssl1.1

COPY target/release/cryptopublisher /crypto-publisher

WORKDIR /
CMD ["/crypto-publisher"]

FROM rust:1.82-slim-bullseye AS builder

RUN useradd -ms /bin/bash app-user

RUN apt update \
    && apt install --yes cmake clang build-essential libavahi-core-dev libavahi-client-dev

COPY . /opt/local/builder
WORKDIR /opt/local/builder

RUN cargo build --release

FROM debian:bullseye-slim AS app

RUN apt update \
    && apt install --yes libavahi-client3

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /etc/passwd /etc/passwd

COPY --from=builder /opt/local/builder/target/release/ractor-poc /usr/bin/app

#USER app-user

CMD ["/usr/bin/app", "run"]
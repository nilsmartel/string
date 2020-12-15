FROM rust as builder
WORKDIR /shell_string

COPY . .

RUN  cargo build --release

FROM alpine
COPY --from=builder /shell_string/target/release/string /usr/bin/string


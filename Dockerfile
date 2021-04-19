# use rust Docker image only to build binary
FROM rust as builder
WORKDIR /shell_string

COPY . .

# build with max optimisations
RUN  cargo build --release

# use very small container to ship out binary with
FROM alpine
COPY --from=builder /shell_string/target/release/string /usr/bin/string
CMD ["string"]

FROM rust:1.61.0 as builder

# create a new empty shell project
RUN USER=root
WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

# our final base
FROM alpine:3.17
#
RUN apk update && apk add gcompat gcc fontconfig freetype
#
## copy the build artifact from the build stage
COPY --from=builder /app/target/release/ckb-log-analyzer /bin/ckb-log-analyzer
#
# set the startup command to run your binary
ENTRYPOINT [ "/bin/ckb-log-analyzer" ]


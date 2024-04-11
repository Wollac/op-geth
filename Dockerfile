# Support setting various labels on the final image
ARG COMMIT=""
ARG VERSION=""
ARG BUILDNUM=""

# Build the Stark verifier in a Rust container
FROM rust:1.75 AS rust-builder
ADD ./core/vm/lib /rustlib
RUN cd /rustlib && cargo build --release --lib

# Build Geth in a stock Go builder container
FROM golang:1.21 as builder

RUN apt-get update -y && apt-get install --no-install-recommends -y -q build-essential ca-certificates git

# Get dependencies - will also be cached if we won't change go.mod/go.sum
COPY go.mod /go-ethereum/
COPY go.sum /go-ethereum/
RUN cd /go-ethereum && go mod download

ADD . /go-ethereum
COPY --from=rust-builder /rustlib/target/release/libstark_verifier.a /go-ethereum/core/vm/lib/libstark_verifier.a
RUN cd /go-ethereum && go run build/ci.go install -static ./cmd/geth

# Pull Geth into a second stage deploy alpine container
FROM alpine:latest

RUN apk add --no-cache ca-certificates
COPY --from=builder /go-ethereum/build/bin/geth /usr/local/bin/

EXPOSE 8545 8546 30303 30303/udp
ENTRYPOINT ["geth"]

# Add some metadata labels to help programatic image consumption
ARG COMMIT=""
ARG VERSION=""
ARG BUILDNUM=""

LABEL commit="$COMMIT" version="$VERSION" buildnum="$BUILDNUM"

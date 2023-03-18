####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

WORKDIR /zkrat

COPY ./ .

RUN cargo build -p server --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:buster-slim

# Create unprivileged user
ENV USER=zkrat
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /zkrat

# Copy our build
COPY --from=builder /zkrat/target/release/server ./

# Use an unprivileged user
USER zkrat:zkrat

CMD ["/zkrat/server"]

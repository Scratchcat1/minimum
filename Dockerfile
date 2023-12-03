####################################################################################################
## Builder
####################################################################################################
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev libressl-dev pkgconfig

WORKDIR /m-rs

COPY . .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import ca-certificates from builder
COPY --from=builder /usr/share/ca-certificates /usr/share/ca-certificates
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

# Copy our build
COPY --from=builder /m-rs/target/release/m_rs /usr/local/bin/m_rs

# Use an unprivileged user.
RUN adduser --home /nonexistent --no-create-home --disabled-password m-rs
USER m-rs

# Tell Docker to expose port 8080
EXPOSE 9080

# Run a healthcheck every minute to make sure m-rs is functional
HEALTHCHECK --interval=1m --timeout=3s CMD wget --spider --q http://localhost:9080 || exit 1

CMD ["m_rs"]
####################################################################################################
## Builder
####################################################################################################
FROM rust AS builder

#RUN apk add --no-cache libc-dev openssl-dev build-base musl-dev pkgconfig perl openssl

WORKDIR /minimum

COPY . .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:stable-slim

RUN apt-get update && apt-get install -y \
  openssl ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Copy our build
COPY --from=builder /minimum/target/release/m_rs /usr/local/bin/m_rs

# Use an unprivileged user.
RUN adduser --home /nonexistent --no-create-home --disabled-password minimum
USER minimum

# Tell Docker to expose port 8080
EXPOSE 9080

# Run a healthcheck every minute to make sure minimum is functional
HEALTHCHECK --interval=1m --timeout=3s CMD wget --spider --q http://localhost:9080 || exit 1

CMD ["m_rs"]

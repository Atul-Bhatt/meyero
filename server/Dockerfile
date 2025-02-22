From rust:1.85-alpine
RUN apk add --no-cache clang lld musl-dev git perl openssl-libs-static pkgconf openssl-dev  make
WORKDIR /app
COPY . .
ARG SQLX_OFFLINE=true
ENV SQLX_OFFLINE=true  
RUN cargo build --release
ENTRYPOINT ["./target/release/server"]
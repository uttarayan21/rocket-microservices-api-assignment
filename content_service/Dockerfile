FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim as runner
COPY --from=builder /app/target/release/content_service /usr/local/bin/content_service
WORKDIR /app
EXPOSE 8000
RUN apt-get update && apt-get install postgresql -y

# COPY init-user-service.sh /docker-entrypoint-initdb.d/init-user-service.sh
CMD ["content_service"]

FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim as runner
COPY --from=builder /app/target/release/user_interaction_service /usr/local/bin/user_interaction_service
WORKDIR /app
EXPOSE 8000
RUN apt-get update && apt-get install postgresql -y

# COPY init-user-service.sh /docker-entrypoint-initdb.d/init-user-service.sh
CMD ["user_interaction_service"]

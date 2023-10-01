FROM rust:1.70
WORKDIR /app
COPY . .
RUN cargo build
EXPOSE 9090
CMD ["cargo", "run"]

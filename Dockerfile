FROM rust:1.78-slim
WORKDIR /app
COPY . .
CMD ["cargo", "run", "--release"]
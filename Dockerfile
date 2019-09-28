FROM rust:latest

RUN rustup update nightly && rustup default nightly

COPY Cargo.toml .
COPY src/ ./src/
RUN cargo fetch

RUN ["cargo", "build", "-Z", "unstable-options", "--out-dir", "/output"]

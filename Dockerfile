# Based on https://dylananthony.com/blog/how-to-write-a-github-action-in-rust/
FROM rust:1.89 as build

# Create empty Rust project first, which apparently allows Docker to cache more efficiently
RUN USER=root cargo new --bin release-from-changelog
WORKDIR /release-from-changelog

# Run build with dependency manifests only, so Docker can cache dependencies
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release
RUN rm src/*.rs

# Now copy source files to build full binary
COPY ./src ./src
RUN rm ./target/release/deps/release_from_changelog*
RUN cargo build --release

# Switch to smaller base image for actually running the action
FROM gcr.io/distroless/cc AS runtime

# Copy binary from build stage
COPY --from=build /release-from-changelog/target/release/release-from-changelog .

# Run binary
ENTRYPOINT ["/release-from-changelog"]

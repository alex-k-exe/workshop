FROM rust:1.86.0-slim-bullseye AS build

# create a new empty shell project
RUN USER=root cargo new --bin workshop
WORKDIR /workshop

# copy over your manifests
COPY ./Cargo.lock ./Cargo.toml ./

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/workshop*
RUN cargo build --release

# our final base
FROM rust:1.49

# copy the build artifact from the build stage
COPY --from=build /workshop/target/release/workshop .

# set the startup command to run your binary
CMD ["./workshop"]

# https://blog.mileswatson.net/url-shortener-with-rust-svelte-and-aws-4-dockerizing

########################
# Build the Rust source into an executable
########################

# select a starting image to build off
FROM rust as server

# set our working directory in the container as /app
WORKDIR /app

# copy the vital cargo information into the container
COPY Cargo.lock .
COPY Cargo.toml .
# cache dependencies by creating an empty
# lib.rs file and building the project
RUN mkdir src
RUN echo "// empty file" > src/lib.rs
RUN cargo build --release

# now copy the code over
COPY src src

# build the release
RUN cargo install --offline --path .


########################
# Copy into an image
########################

# use a slim image for actually running the container.
FROM debian:buster-slim

RUN mkdir /app

# allow requests to port 80 and 443 (http and https)
EXPOSE 80
EXPOSE 443
EXPOSE 8080

# install the program onto the current image
COPY --from=server /usr/local/cargo/bin/recipe-server /app/recipe-server

# this command is run when we actually start the container
CMD ["/app/recipe-server"]

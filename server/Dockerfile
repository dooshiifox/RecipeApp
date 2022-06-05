# https://blog.mileswatson.net/url-shortener-with-rust-svelte-and-aws-4-dockerizing
# https://hub.qovery.com/guides/tutorial/how-to-deploy-a-rust-rest-api-application-on-aws-with-ease/

########################
# Build the Rust source into an executable
########################

# select a starting image to build off
FROM rust:latest as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=app
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# set our working directory in the container as /app
WORKDIR /app

# ===============================================
# download dependencies by creating an empty
# lib.rs file and building the project
# ===============================================

# First copy over ActixApiMacros so it doesnt whine later.
RUN mkdir actix-api-macros
RUN mkdir actix-api-macros/src
WORKDIR /app/actix-api-macros
COPY actix-api-macros/Cargo.lock .
COPY actix-api-macros/Cargo.toml .
RUN echo "// empty file" > src/lib.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# Then copy over the main server's package information.
WORKDIR /app
RUN mkdir src
COPY Cargo.lock .
COPY Cargo.toml .
RUN echo "// empty file" > src/lib.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# now copy the code over
# Pay attention to `.dockerignore`!
COPY ./ .

WORKDIR /app
RUN rm -rf target
RUN cargo build --release --offline --target x86_64-unknown-linux-musl


########################
# Copy into an image
########################

# use a slim image for actually running the container.
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# allow requests to port 80 and 443 (http and https)
EXPOSE 80
EXPOSE 443
EXPOSE 8080

# install the program onto the current image
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/recipe-server ./

# this command is run when we actually start the container
CMD ["/app/recipe-server"]

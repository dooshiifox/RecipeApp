# 2022 Nutriblocks Kids Recipe App - Server

The server code repository for my 2022 NCEA assessment (& scholarship?) project.
Built for [Nutriblocks](https://nutriblocks.co.nz/).

## Layout

-   `src/` - The source code for the back-end.
-   `target/` - The compiled back-end code. Can pretty much be ignored. Uncommitted but built by `$ cargo run`Rocket has launched from http://0.0.0.0:80
-   `Cargo.lock` - Dependencies of the project. Do not untrack! This is important so that everyone has the same dependencies instead of different versions of them!
-   `Cargo.toml` - Cargo / Rust [config file](https://doc.rust-lang.org/cargo/reference/manifest.html).
-   `docker-compose.yml` - Docker compose setup. While we could probably run the server in its own container without requiring compose, it's still good to have for the future.
-   `Dockerfile` - The main Dockerfile for the server. Compiles the code and then starts the server in release mode.

## Docker

[Docker](https://www.docker.com/products/docker-desktop) is software that's similar to a virtual machine on your computer, but more lightweight. If you have experience with this type of thing, you'll know containerizing is a pain to set up and maintain, but unforunately required if we want to deploy this code to AWS. It does mean we can test in a production-like environment before deployment by simply starting the container.

Fortunately, we don't need to rebuild and restart our container for every code change.

For local development, kill the server and run `$ cargo run` whenever you make a change.

For production testing, kill the container with `$ docker compose down` and start it back up again with `$ docker compose up --build --remove-orphans`. On Linux, `sudo` may be requried if you have [not set up groups yet](https://docs.docker.com/engine/install/linux-postinstall/#manage-docker-as-a-non-root-user).

&nbsp;

# How to Run the Code Locally

# To build the site in a development environment

Firstly, make sure you have `rust` installed. If you do not, download it from [here](https://www.rust-lang.org/tools/install).

```
$ rustc -V
rustc 1.58.1 (db9d1b20b 2022-01-20)
$ cargo -V
cargo 1.58.0 (f01b232bc 2022-01-19)
```

## Starting the code

Run the following to start the server in development.

```
$ cargo run
```

It will likely try start compiling code. This is gonna be a few minutes, so just sit back and wait for it to say `Starting Actix-web server on http://0.0.0.0:8080`. Once it does, visit that URL and the site should be working!

# To build the site in a production environment

Firstly, make sure Docker is installed.

```
$ docker -v
Docker version 20.10.12, build e91ed57
$ docker compose
Usage:  docker compose [OPTIONS] COMMAND

Docker Compose
[...]
Run 'docker compose COMMAND --help' for more information on a command.
```

If you don't have it installed, or it isn't at least version 20, you can download Docker from [here](https://www.docker.com/products/docker-desktop).

If it is installed, open a terminal or command-line to the root folder of the project, run the following command, and wait. It'll take a while - especially if you've not run it before.

```
$ docker compose down
$ docker compose up --build --remove-orphans
```

When it says `Starting Actix-web server on http://0.0.0.0:80`, go to [`localhost`](http://localhost) in your browser and it should show the site!

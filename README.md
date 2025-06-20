<div align="center">

[![Discord]][Discord-invite]
<img src="https://img.shields.io/static/v1?label=Status&message=Early%20Development&color=blue">
[![FAQ-shield]][FAQ]

[FAQ-shield]: https://img.shields.io/badge/Frequently_Asked_Questions_(FAQ)-ff62bd
[FAQ]: https://github.com/polyphony-chat/.github/blob/main/FAQ.md

</div>

<p align="center">
  
  <img width="128" src="https://github.com/polyphony-chat/branding/blob/main/logos/polyphony-symfonia-transparent-8bit.png?raw=true" alt="The Symfonia logo. a dark, square background with rounded edges. on this background, there are four vertically stacked, purple lines. The lines are all vaguely u-shaped and resemble sound waves being emitted into one direction, with the lower lines being thicker and wider than the upper lines." />
  <h1 align="center">Symfonia</h1>
</p>

## About

This is a WIP implementation of a polyproto-chat compatible Server in Rust.

!!! info

      This server is still in early development and thus not yet usable.



This repository contains:
A partial implementation of:

- [HTTP API Server](/src/crates/symfonia-api)
- [WebSocket Gateway Server](/src/crates/symfonia-gateway)
- [Database Models](/src/crates/util/migrations)

## Local Development/Test Environment

Whether you are using Docker or not, you will need to have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [git](https://git-scm.com/downloads)
- [`sqlx-cli`](https://crates.io/crates/sqlx-cli)

You will also have to run `git submodule update` after cloning the repository.

If your development environment is hosted on a Windows machine, please additionally make sure that
you have a bash shell available to execute pre-commit hooks. This can be done by installing
[Git Bash](https://git-scm.com/downloads) or
[Windows Subsystem for Linux 2](https://learn.microsoft.com/en-us/windows/wsl/install) and, additionally,
configuring your IDE correctly.

See the instructions below for guidance on how to run the project.

### Non-Docker

1. Install and host a [PostgreSQL database](https://www.postgresql.org/download/)
2. Create a new database, and a user that has full access to that database
3. Create a `.env` file in the root of the project with the following contents:

```env
DATABASE_HOST=[ip/domain of your Postgres database]
DATABASE_PORT=[Postgres port, usually 5432]
DATABASE_USERNAME=[Your Postgres username]
DATABASE_PASSWORD=[Your Postgres password]
DATABASE_NAME=[Your Postgres database name]
API_BIND=[ip:port to bind the HTTP API server to. Defaults to 0.0.0.0:3001 if not set]
GATEWAY_BIND=[ip:port to bind the Gateway server to. Defaults to 0.0.0.0:3003 if not set]
```

5. Install the sqlx CLI with `cargo install sqlx-cli`
6. Run `cargo sqlx migrate run` from within the project directory to run the migrations
7. Run the project with `cargo run`.

### Docker

1. Copy the `compose-example.env` file to `.env` in the root of the compose project and fill in the values
   to your liking.
2. Adjust ports in `docker-compose.yml` if needed.
3. Run `docker compose up --build`.

Code changes will require you to restart the container with `docker compose up --build`. If you want
to reset to a fully clean state, run `docker compose down -v`.

[Discord]: https://dcbadge.vercel.app/api/server/m3FpcapGDD?style=flat
[Discord-invite]: https://discord.com/invite/m3FpcapGDD
[build-shield]: https://img.shields.io/github/actions/workflow/status/polyphony-chat/symfonia/rust.yml?style=flat
[build-url]: https://github.com/polyphony-chat/symfonia/blob/main/.github/workflows/rust.yml

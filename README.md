# Ttyh Master

This is the new implementation of the [minecraft-master](https://github.com/betrok/minecraft-master)
but written in Rust. It is created just for fun as an education project.

It can be built to work with `SQLite` or `MySQL` databases.

## Setup

It requires to install the `sqlx-cli` first:

```
cargo insatll sqlx-cli
```

Then you need to set up the `.env` file:

| Database | Command                                                         |
|----------|-----------------------------------------------------------------|
| SQLite   | `echo "DATABASE_URL=sqlite://workdir/db_name.db" > .env`        |
| MySQL    | `echo "DATABASE_URL=mysql://user:password@host/db_name" > .env` |

After that you can crate the database:

```
sqlx database create
```

Now you can execute migrations:

| Database | Command                                       |
|----------|-----------------------------------------------|
| SQLite   | `sqlx migrate run --source migrations/sqlite` |
| MySQL    | `sqlx migrate run --source migrations/mysql`  |

The last step is to build the app:

| Database | Command                                   |
|----------|-------------------------------------------|
| SQLite   | `cargo build --release --features sqlite` |
| MySQL    | `cargo build --release --features mysql`  |

Don't forget to set up the working directory before run.
You should set up the configuration file, and prepare some assets (server key, default skin).
For details see the [example config](workdir/config.toml).

Sample workdir structure:

```
workdir/
├── assets/
│   ├── capes/
│   ├── skins/
│   └── default_skin
├── keypairs/
├── config.toml
├── example.db
└── server_key.pem
```

## Internal API

The internal API is very basic and allows to create, get and update players:

| Action              | Endpoint                  | Request Payload                              | Responses                                                                |
|---------------------|---------------------------|----------------------------------------------|--------------------------------------------------------------------------|
| Create a new player | `POST /ttyh/player`       | [CreatePlayerRequest](src/payload/bridge.rs) | `400`, `500`, `200`                                                      |
| Get player info     | `GET /ttyh/player/:name`  |                                              | `400`, `404`, `500`, `200` + [PlayerInfoResponse](src/payload/bridge.rs) |
| Update player info  | `POST /ttyh/player/:name` | [UpdatePlayerRequest](src/payload/bridge.rs) | `400`, `404`, `500`, `200`                                               |

Every endpoint should be called with the authorisation
header: `Authorization: Bearer token-defined-in-the-config`.

Response codes follow the standard HTTP convention:

- `400` - Authorisation header is not set or invalid
- `404` - Player not found
- `500` - Database query error
- `200` - Operation successful

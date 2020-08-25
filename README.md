# cantaloupe 🍈

A rpm repository with a *proper* (whatever that means) backend.

## Attention

WIP! Don't even try to run this. It is broken for sure.

Once it will be ready...


## Install


### cargo

```sh
cargo install cantaloupe; cantaloupe
```

### rpm repository

```sh
dnf copr enable drahnr/cantaloupe
dnf install cantaloupe micro
$EDITOR /etc/cantaloupe.conf
systemctl enable cantaloupe.service
```

### containerized (minimal)

```sh
podman run -it quay.io/drahnr/cantaloupe
```

## Storage

Requires a running postgres instance, see [configuration].


## Configuration

By default, tries to connect to a local database exposed at `127.0.0.1:5431/cantaloupe`.


Configuration files are also supported in toml fmt and are checked in the following ordered sequence:

1. `${PWD}/.config/cantaloupe.conf`
2. `${HOME}/.config/cantaloupe/cantaloupe.toml`
3. `/etc/cantaloupe/cantaloupe.conf`
3. `/etc/cantaloupe.conf`


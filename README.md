# cantaloupe

A rpm repository with a proper backend.

## Attention

WIP! Don't even try to run this. It is broken for sure.

Once it will be ready...

### Requires

 * a running postgres instance

### HowTo

There are a couple of different ways to run it:

rustic: `cargo install cantaloupe; cantaloupe` 
rpmic: `dnf copr enable drahnr/cantaloupe; dnf install cantaloupe micro; mciro /etc/cantaloupe.conf; systemctl enable cantaloupe.service`
container: `runc docker://...TBD`
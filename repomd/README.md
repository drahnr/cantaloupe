# repomd - rpm repo metdata

**WIP** A gigantic piece of in porgres and unstructured, mostly broken code **WIP**
 
## Client

A client spec that includes the a simple toml-like format describing the remote end of the repository.

## Server

* Provide `repo metadata` compliant metadata from an arbitrary source via `struct Repo`.
* Load `createrepo` repositories into memory

## Spec

Follows  https://en.opensuse.org/openSUSE:Standards_Rpm_Metadata and assures real life functionality
with guaranteed compat with `Fedora 32+`.`

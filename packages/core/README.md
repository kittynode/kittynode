# core

This is the core library powering [kittynode-cli](../cli/README.md) and [kittynode-gui](../gui/README.md). It's designed to be used as a module for any project that needs to manage decentralized node infrastructure.

## Project structure

> Generated with `eza --tree --level 3 --only-dirs packages/core`

```
packages/core
└── src
   ├── application
   ├── domain
   ├── infra
   └── manifests
```

#### application (public)

The application folder contains use cases for consumers of the library. Along with the domain folder, it represents the entire surface of the external API.

#### domain (public)

The domain folder contains struct definitions for the domains.

#### infra (internal)

The infra folder contains modules that interface with integration boundaries outside of the program. For example the file system, containerization tools, or networking. They don't represent any consumer use cases, so this module is internal.

#### manifests (internal)

The manifests folder contains declarative manifests that describe packages which are supported by `kittynode-core`. These manifests are also internal details, but they should install with good default values.

## Delivery

#### Rust crate

`kittynode-core` is available as a Rust crate to use in Rust programs.

#### JavaScript module

This is not currently available, but a path we are interested in. The JavaScript module will naturally not support all the packages that the Rust crate does, but it should support packages that run on wasm. For example, the Helios light client, or any JavaScript node implementations.

## Documentation

The documentation for `kittynode-core` is currently under development.

# Architecture

## Overview

Kittynode is a cross-platform application that runs p2p node software. The goal is to be easy to use, to increase participation in decentralized networks.

## Components

Kittynode is architected as a core library, which are consumed by two frontends: a CLI and a GUI. Everything is written in Rust in order to increase safety and compatibility (MacOS, Linux, Windows).

Aside from the end user which is running a node, there is one more important user: package developers. Kittynode is designed to be lean in the core library, and extended by package developers. For example, one popular package will be a Reth package, which will run a reth node. Another package will be a Lighthouse package, which will run a lighthouse node. There also might be cases where users want to compose packages, or install an all-in-one package (reth + lighthouse).

It's possible that all packages run on Docker, but I also think it's important as a design goal to make sure we can refactor kittynode in the future to easily support static binary executables, or other types of packaging aside from Docker.

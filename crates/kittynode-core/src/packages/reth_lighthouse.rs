use bollard::secret::PortBinding;
use eyre::Result;
use std::collections::HashMap;

use crate::{
    file::kittynode_path,
    package::{Binding, Container, Package, PackageDefinition},
};

pub struct RethLighthouse;

impl PackageDefinition for RethLighthouse {
    const NAME: &'static str = "Reth + Lighthouse (Holesky)";

    fn get_package() -> Result<Package> {
        let kittynode_path = kittynode_path()?;
        let jwt_path = kittynode_path.join("jwt.hex");

        Ok(Package {
            description: "This package installs a Reth execution client and a Lighthouse consensus client on the Holesky network with Docker.",
            network_name: "reth-lighthouse-holesky-network",
            containers: vec![
                Container {
                    name: "reth-node",
                    image: "ghcr.io/paradigmxyz/reth",
                    cmd: vec![
                        "node",
                        "--chain",
                        "holesky",
                        "--metrics",
                        "0.0.0.0:9001",
                        "--authrpc.addr",
                        "0.0.0.0",
                        "--authrpc.port",
                        "8551",
                    ],
                    port_bindings: HashMap::from([
                        (
                            "9001/tcp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("9001".to_string()),
                            }],
                        ),
                        (
                            "30303/tcp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("30303".to_string()),
                            }],
                        ),
                        (
                            "30303/udp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("30303".to_string()),
                            }],
                        ),
                    ]),
                    volume_bindings: vec![
                    Binding {
                        source: "rethdata".to_string(),
                        destination: "/root/.local/share/reth/holesky".to_string(),
                        options: None,
                    }
                    ],
                    file_bindings: vec![
                    Binding {
                        source: jwt_path.display().to_string(),
                        destination: "/root/.local/share/reth/holesky/jwt.hex".to_string(),
                        options: Some("ro".to_string()),
                        }
                    ],
                },
                Container {
                    name: "lighthouse-node",
                    image: "sigp/lighthouse",
                    cmd: vec![
                        "lighthouse",
                        "--network",
                        "holesky",
                        "beacon",
                        "--http",
                        "--http-address",
                        "0.0.0.0",
                        "--checkpoint-sync-url",
                        "https://checkpoint-sync.holesky.ethpandaops.io",
                        "--execution-jwt",
                        "/root/.lighthouse/holesky/jwt.hex",
                        "--execution-endpoint",
                        "http://reth-node:8551",
                    ],
                    port_bindings: HashMap::from([
                        (
                            "9000/tcp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("9000".to_string()),
                            }],
                        ),
                        (
                            "9000/udp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("9000".to_string()),
                            }],
                        ),
                        (
                            "9001/udp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("9001".to_string()),
                            }],
                        ),
                        (
                            "5052/tcp",
                            vec![PortBinding {
                                host_ip: Some("127.0.0.1".to_string()),
                                host_port: Some("5052".to_string()),
                            }],
                        ),
                    ]),
                    volume_bindings: vec![],
                    file_bindings: vec![
                        Binding {
                            source: kittynode_path.join(".lighthouse").to_string_lossy().to_string(),
                            destination: "/root/.lighthouse".to_string(),
                            options: None,
                        },
                        Binding {
                            source: jwt_path.to_string_lossy().to_string(),
                            destination: "/root/.lighthouse/holesky/jwt.hex".to_string(),
                            options: Some("ro".to_string()),
                        }
                    ],
                }
            ]
        })
    }
}

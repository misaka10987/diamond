# Diamond

Just a daemon.

![GitHub License](https://img.shields.io/github/license/misaka10987/diamond)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/misaka10987/diamond/rust.yml)
![Crates.io Version](https://img.shields.io/crates/v/diamond?label=lib)
![Crates.io Version](https://img.shields.io/crates/v/diamond-server?label=server)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/misaka10987/diamond)

*Diamond* is a modern, cross-platform init system. It targets to become a [systemd](https://systemd.io/) replacement that does nothing else but service initialization and management.

Diamond uses a dependency-based mechanism like systemd to perform parallelized starting. It parents all the service processes and provides RESTful APIs via HTTP over UDS to operate with them.

Diamond is implemented using the performant [tokio](https://crates.io/crates/tokio) asynchronous runtime. 100% written in safe Rust.

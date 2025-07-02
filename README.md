# Diamond

Just a daemon.

*Diamond* is a modern, cross-platform init system. It targets to become a [systemd](https://systemd.io/) replacement that does nothing else but service initialization and management.

Diamond uses a dependency-based mechanism like systemd to perform parallelized starting. It parents all the service processes and provides RESTful APIs via HTTP over UDS to operate with them.

Diamond is implemented using the performant [tokio](https://crates.io/crates/tokio) asynchronous runtime. 100% written in safe Rust.

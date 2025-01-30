## 🔥 libfireparse

libfireparse is a Rust library for parsing firewall configuration files and converting them into a unified format compatible with the NullNet platform.

### Usage
```rust
use libfireparse::Parser;

let xml = r#"<pfsense><aliases></aliases><filter></filter></pfsense>"#;
match Parser::parse("pfsense", xml) {
    Ok(config) => println!("Parsed {} rules and {} aliases", config.rules.len(), config.aliases.len()),
    Err(err) => eprintln!("Error: {:?}", err),
}
```

### Supported platforms
- [x] pfSense
- [ ] OPNsense

### License
MIT
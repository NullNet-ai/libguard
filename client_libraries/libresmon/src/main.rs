use nullnet_libresmon::poll_system_resources;

fn main() {
    let resource_rx = poll_system_resources(1000);
    loop {
        match resource_rx.recv_blocking() {
            Ok(resources) => {
                let _ = std::process::Command::new("clear").status();
                println!("{resources}");
            }
            Err(e) => {
                eprintln!("Error receiving system resources: {e}");
            }
        }
    }
}

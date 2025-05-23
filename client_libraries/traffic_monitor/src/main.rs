// sample program to demonstrate how to use the traffic_monitor library

use nullnet_traffic_monitor::{MonitorConfig, monitor_devices};

fn main() {
    let monitor_config = MonitorConfig {
        addr: "127.0.0.1".to_string(),
        snaplen: 96,
    };
    let rx = monitor_devices(&monitor_config);

    loop {
        if let Ok(packet) = rx.recv_blocking() {
            println!("Received packet on {}", packet.interface);
        }
    }
}

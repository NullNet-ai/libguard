// sample program to demonstrate how to use the traffic_monitor library

use nullnet_traffic_monitor::{monitor_devices, MonitorConfig};

fn main() {
    let monitor_config = MonitorConfig {
        addr: "127.0.0.1".to_string(),
        snaplen: 96,
    };
    let rx = monitor_devices(&monitor_config);

    loop {
        if let Ok(packet) = rx.recv() {
            println!("Received packet on {}", packet.interface);
        }
    }
}
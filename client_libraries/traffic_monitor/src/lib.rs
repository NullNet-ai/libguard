use chrono::Utc;
use pcap::Device;
use std::net::ToSocketAddrs;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

/// Configuration for the network traffic monitor
pub struct MonitorConfig {
    /// The address to filter out from the captured packets (i.e., the server's IP address)
    pub addr: String,
    /// The maximum number of bytes to capture per packet
    pub snaplen: i32,
}

/// Information about a captured packet
pub struct PacketInfo {
    /// The interface name the packet was captured on
    pub interface: String,
    /// The raw packet data
    pub data: Vec<u8>,
    /// The link type of the packet
    pub link_type: i32,
    /// The timestamp of the packet capture
    pub timestamp: String,
}

/// Monitor network traffic on all available network devices
///
/// # Arguments
/// * `monitor_config` - The configuration for the network traffic monitor
///
/// # Returns
/// A receiver channel for receiving captured packets
#[must_use]
pub fn monitor_devices(monitor_config: &MonitorConfig) -> Receiver<PacketInfo> {
    let (tx, rx) = mpsc::channel();

    let bpf_program = bpf_program(&monitor_config.addr);
    for device in Device::list().into_iter().flatten() {
        let tx = tx.clone();
        let snaplen = monitor_config.snaplen;
        let bpf_program = bpf_program.clone();
        thread::spawn(move || {
            monitor_device(device, &tx, snaplen, &bpf_program);
        });
    }

    rx
}

fn monitor_device(device: Device, tx: &Sender<PacketInfo>, snaplen: i32, bpf_program: &str) {
    let device_name = device.name.clone();

    let Ok(mut cap) = pcap::Capture::from_device(device)
        .expect("capture initialization error")
        .promisc(true)
        .snaplen(snaplen) // limit stored packets slice dimension (to keep more in the buffer)
        .immediate_mode(true) // parse packets ASAP!
        .open()
    else {
        return;
    };

    cap.filter(bpf_program, true).expect("PCAP filter error");

    let link_type = cap.get_datalink().0;

    loop {
        if let Ok(p) = cap.next_packet() {
            let packet = PacketInfo {
                interface: device_name.clone(),
                data: p.data[..].to_vec(),
                link_type,
                timestamp: Utc::now().to_rfc3339(),
            };
            tx.send(packet).unwrap_or_default();
        }
    }
}

fn bpf_program(addr: &str) -> String {
    let ip_addr = format!("{addr}:0")
        .to_socket_addrs()
        .expect("Failed to resolve address")
        .next()
        .expect("Failed to get address")
        .ip()
        .to_string();

    let bpf_program = format!("host not {ip_addr}");
    println!("BPF Program: {bpf_program}");
    bpf_program
}

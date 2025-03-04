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

    let mut cap = match pcap::Capture::from_device(device)
        .map(|c| c.promisc(true).snaplen(snaplen).immediate_mode(true).open())
    {
        Ok(Ok(cap)) => cap,
        Ok(Err(err)) | Err(err) => {
            log::warn!(
                "Failed to initialize capture on {device_name}: {err}. Aborting monitoring..."
            );
            return;
        }
    };

    if let Err(err) = cap.filter(bpf_program, true) {
        log::error!("PCAP filter error on {device_name}: {err}. Aborting monitoring...");
        return;
    }

    let mut savefile = if cfg!(feature = "export-pcap") {
        let file_name = format!("./{device_name}.pcap");
        let res = cap.savefile(&file_name);
        match res {
            Ok(savefile) => Some(savefile),
            Err(err) => {
                log::error!("Failed to create savefile '{file_name}': {err}");
                None
            }
        }
    } else {
        None
    };

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
            // Save packet to file
            if let Some(file) = savefile.as_mut() {
                file.write(&p);
            }
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
    log::info!("BPF Program: {bpf_program}");
    bpf_program
}

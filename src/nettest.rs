use pcap::Capture;

pub fn main() {
    let mut cap = Capture::from_device("wlo1")
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    while let Ok(packet) = cap.next_packet() {
        println!("received packet! {:?}", packet);
    }
}

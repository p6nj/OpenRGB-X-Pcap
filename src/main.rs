use openrgb::OpenRGB;
use pcap::Capture;
use rgb::RGB;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = OpenRGB::connect().await?;
    client.set_name("Rust").await?;

    let led_number = client.get_controller(0).await?.leds.len();

    let mut cap = Capture::from_device("wlo1")
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    while let Ok(packet) = cap.next_packet() {
        let size = packet.data.len();
        for i in 0..match size * 3 < led_number * 3 {
            true => size * 3,
            false => led_number * 3,
        } {
            client
                .update_led(
                    0,
                    i.try_into().unwrap(),
                    RGB {
                        r: packet.data[i],
                        g: packet.data[i + 1],
                        b: packet.data[i + 2],
                    },
                )
                .await?;
        }
    }

    Ok(())
}

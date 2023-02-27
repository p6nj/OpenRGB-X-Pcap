use std::{env::args, error::Error};

use anyhow::{Context, Result};
use openrgb::OpenRGB;
use pcap::{Capture, Device};
use rgb::RGB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("ico/keyboard icon.ico");
        res.compile()
            .context("Icon not found, should be in 'ico/keyboard icon.ico'")?;
    }

    let client = OpenRGB::connect().await.context(
        "Couldn't connect to OpenRGB. Is the server running and listening on port 6742?",
    )?;
    // TODO: change client name when repo is public
    client
        .set_name("Rust")
        .await
        .context("Can't set the client's name... What??")?;

    // Controller in use. Defaults to 0.
    let mut controller = 0u8;
    {
        // friendly arg scan
        let mut activate = false;
        let controller_option = "-c".to_string();
        for arg in args().take(args().len() - 1).skip(1) {
            if activate {
                controller = arg
                    .parse()
                    .context("Controller option value should be a single digit")?;
                break;
            } else if arg == controller_option {
                activate = true;
            }
        }
    }
    let led_number = client
        .get_controller(0)
        .await
        .with_context(|| format!("Failed to reach controller {controller}"))?
        .leds
        .len();

    let mut cap = Capture::from_device(
        Device::lookup()
            .context("Device lookup failed")?
            .context("No device available")?,
    )
    .context("Cannot sniff from default device")?
    .immediate_mode(true)
    .snaplen(3 * led_number as i32) // restrict the size of captured packet data
    .open()
    .context("Capture activation failed (if you got an 'Operation not permitted' error from libpcap on Linux, run: 'sudo setcap cap_net_raw,cap_net_admin=eip EXECUTABLE')")?;

    while let Ok(packet) = cap.next_packet() {
        // high-perf-only zone
        let mut data = Vec::from(packet.data);
        data.fill(0);
        for i in 0..led_number {
            client
                .update_led(
                    0,
                    i.try_into().unwrap(),
                    RGB {
                        r: packet.data[i],
                        g: packet.data[i * 3 + 1],
                        b: packet.data[i * 3 + 2],
                    },
                )
                .await?;
        }
    }

    Ok(())
}

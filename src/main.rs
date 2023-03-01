use std::{env::args, error::Error, io::Write};

use anyhow::{Context, Result};
use openrgb::OpenRGB;
use pcap::{Capture, Device, Inactive};
use rgb::RGB;
use std::io::{stdin, stdout};

fn dev_picker() -> Result<Device> {
    let mut line = String::new();
    let list = Device::list().context("Can't enumerate devices")?;
    for (i, dev) in list.iter().enumerate() {
        println!(
            "{}. {}",
            i,
            dev.desc.as_ref().or_else(|| Some(&dev.name)).unwrap()
        );
    }
    print!("Choose a device: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");
    Ok(
        match list.get(
            line.trim()
                .parse::<usize>()
                .context("Choice isn't an integer")?,
        ) {
            Some(d) => d.clone(),
            _ => unreachable!(),
        },
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cap: Result<Capture<Inactive>, pcap::Error>;
    if cfg!(target_os = "windows") {
        cap = Capture::from_device(dev_picker()?);
    } else {
        cap = Capture::from_device(
            Device::lookup()
                .context("Device lookup failed")?
                .context("No device available")?,
        );
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

    // dbg!(Device::list());

    let mut cap = cap
    .context("Cannot sniff from default device")?
    .immediate_mode(true)
    .snaplen(3 * led_number as i32) // restrict the size of captured packet data
    .open()
    .context("Capture activation failed (if you got an 'Operation not permitted' error from libpcap on Linux, run: 'sudo setcap cap_net_raw,cap_net_admin=eip EXECUTABLE')")?;

    let mut copy = vec![[0u8; 3]; led_number];
    let tries = 3u8;
    for _ in 0..tries {
        while let Ok(packet) = cap.next_packet() {
            // high-perf-only zone
            let mut data = vec![[0u8; 3]; led_number];
            packet
                .data
                .iter()
                .enumerate()
                .for_each(|(i, x)| data[i / 3][i % 3] = *x);
            if copy == data {
                continue;
            }
            copy = data; // move
            for (i, x) in copy.iter().enumerate() {
                client
                    .update_led(
                        0,
                        i as i32,
                        RGB {
                            r: x[0],
                            g: x[1],
                            b: x[2],
                        },
                    )
                    .await
                    .context("Error updating LEDs")?;
            }
        }
    }

    Ok(())
}

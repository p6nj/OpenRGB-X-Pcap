use openrgb::OpenRGB;
use rgb::RGB;
use std::error::Error;
mod nettest;

// #[tokio::main]
// async
fn main() -> Result<(), Box<dyn Error>> {
    // let client = OpenRGB::connect().await?;
    // client.set_name("Rust").await?;

    // client.update_led(0, 0, RGB { r: 255, g: 0, b: 0 }).await?;

    nettest::main();

    Ok(())
}

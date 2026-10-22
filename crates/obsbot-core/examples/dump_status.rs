// Capture the raw 60-byte status struct in Standard and Sport, restoring
// the original speed at the end.
use std::fs::File;

use obsbot_core::xu::commands::set_tracking_speed;
use obsbot_core::xu::{get_status, TrackingSpeed};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::options().read(true).write(true).open("/dev/video0")?;
    let base = get_status(&file)?;
    println!("base speed={:?}", base.tracking_speed);
    print_raw(&base.raw);
    let other = match base.tracking_speed {
        TrackingSpeed::Standard => TrackingSpeed::Sport,
        TrackingSpeed::Sport => TrackingSpeed::Standard,
    };
    set_tracking_speed(&file, other)?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    let changed = get_status(&file)?;
    println!("changed speed={:?}", changed.tracking_speed);
    print_raw(&changed.raw);
    set_tracking_speed(&file, base.tracking_speed)?;
    Ok(())
}

fn print_raw(raw: &[u8; 60]) {
    for chunk in raw.chunks(16) {
        println!(
            "{}",
            chunk
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<Vec<_>>()
                .join(",")
        );
    }
}

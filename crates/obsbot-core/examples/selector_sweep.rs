// Read-only sweep of the vendor XU selectors: GET_LEN on every selector,
// then GET_CUR where the length is plausible. No SET_CUR anywhere.

use std::fs::File;

use obsbot_core::xu::transport::{get_cur, get_len, BUNIT_ID};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dev = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/dev/video0".into());
    let file = File::options().read(true).write(true).open(&dev)?;
    println!("device: {dev} unit: {BUNIT_ID:#04x}");
    for selector in 1u8..=22 {
        match get_len(&file, BUNIT_ID, selector) {
            Ok(len) => {
                print!("selector {selector:#04x}: len={len}");
                if len > 0 && len <= 256 {
                    let mut buf = vec![0u8; usize::from(len)];
                    match get_cur(&file, BUNIT_ID, selector, &mut buf) {
                        Ok(()) => {
                            print!(" cur=");
                            for b in buf.iter().take(64) {
                                print!("{b:02x} ");
                            }
                            if buf.len() > 64 {
                                print!("...");
                            }
                        }
                        Err(e) => print!(" cur-error={e}"),
                    }
                }
                println!();
            }
            Err(e) => println!("selector {selector:#04x}: get_len-error={e}"),
        }
    }
    Ok(())
}

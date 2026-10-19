// Read-only: send a header-only GET frame on selector 0x02 and dump the reply.
// GET frames use flags 0x01 (SET uses 0x25).
use std::fs::File;

use obsbot_core::xu::crc16::crc16;
use obsbot_core::xu::transport::{get_cur, set_cur, BUNIT_ID, SELECTOR_FRAME};

fn get_frame(seq: u16, receiver: u8, cmd: u16) -> Vec<u8> {
    let mut f = [0u8; 12];
    f[0] = 0xaa;
    f[1] = 0x01;
    f[2..4].copy_from_slice(&seq.to_le_bytes());
    f[4..6].copy_from_slice(&12u16.to_le_bytes());
    f[8] = 0x0a;
    f[9] = receiver;
    f[10..12].copy_from_slice(&cmd.to_le_bytes());
    let crc = crc16(&f);
    f[6..8].copy_from_slice(&crc.to_le_bytes());
    f.to_vec()
}

fn query(file: &File, name: &str, cmd: u16) {
    let frame = get_frame(0x0001, 0x02, cmd);
    if let Err(e) = set_cur(file, BUNIT_ID, SELECTOR_FRAME, &frame) {
        println!("{name}: set_cur failed: {e}");
        return;
    }
    std::thread::sleep(std::time::Duration::from_millis(300));
    let mut buf = [0u8; 60];
    match get_cur(file, BUNIT_ID, SELECTOR_FRAME, &mut buf) {
        Ok(()) => {
            print!("{name}: ");
            for b in buf {
                print!("{b:02x} ");
            }
            println!();
        }
        Err(e) => println!("{name}: get_cur failed: {e}"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::options().read(true).write(true).open("/dev/video0")?;
    query(&file, "GET_CAM_STATUS 0xA042", 0xa042);
    query(&file, "GET_EXPOSURE 0x2942", 0x2942);
    query(&file, "GET_GIM_STATE 0x6604", 0x6604);
    query(&file, "GET_BOGUS 0x1234", 0x1234);
    query(&file, "GET_USB_MODE 0x6042", 0x6042);
    query(&file, "GET_UVC_MODE 0xC442", 0xc442);
    Ok(())
}

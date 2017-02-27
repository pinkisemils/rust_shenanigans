extern crate libc;
use libc::size_t;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::mpsc;

#[link(name = "jpeg")]
extern {
    pub fn decode_jpeg(input: *const u8, input_length: size_t, output: *const u8) -> i32;
    pub fn test();
}

pub type Frame = Vec<u8>;
pub fn read_and_send(inf: File, tx: mpsc::SyncSender<Frame>) {
    let mut buffread = BufReader::new(inf);
    let mut read_buff = vec![];
    let mut buff = vec![];
    loop {
        match buffread.read_until(b'\n', &mut read_buff) {
            Ok(_) => {
                if read_buff.len() < 1 {
                    println!("read_buff.len() -> 0");
                    return;
                }
                if matches_bound(&read_buff) {
                    let sendable = buff[..].to_vec();
                    tx.send(trim(sendable));
                    buff.truncate(0);
                } else {
                    buff.extend(&read_buff);
                }
            }
            Err(e) => {
                println!("Encountered error whilst reading from file: {}",e);
            }

        }
        read_buff.truncate(0);

    }
}

static JPEG_START: [u8; 2] = [0xff, 0xd8];
static BOUND: &'static [u8; 13] = b"--myboundary\n";

fn matches_bound(buff: &[u8]) -> bool {
    if buff.len() == 1 { return false; }
    let retval = BOUND.iter().zip(buff.iter()).all(|(a, b)| a == b);
    retval
}


fn trim(mut buff: Vec<u8>) -> Vec<u8> {
    if buff.len() %2 != 0 {
        buff.push(0x00);
    }
    let mut out = Vec::with_capacity(buff.len());
    let mut start = false;
    for bytep in buff.chunks(2) {
        if bytep == JPEG_START {
            start = true;
        }
        if start{
            out.extend(bytep);
        }
    }
    out
}



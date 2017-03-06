extern crate jpeg_decoder as jpeg;
extern crate deco;
extern crate image;

use deco::{decode_jpeg, read_and_send, test, read_decode_send};

use std::io::Read;
use std::sync::mpsc;
use std::thread::spawn;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::min;
use std::io::Write;
use std::time;

type Frame = Vec<u8>;

fn are_same(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    //a.iter().zip(b.iter()).all(|(ai, bi)| {ai == bi})
    let mut retval = true;
    let itr = a.iter().zip(b.iter()).enumerate();
    for (idx, (ia, ib)) in itr {
        let diff = *ia as i32 - *ib as i32;
        if diff > 3 {
            println!("no match at {}, ia - {}, ib - {}", idx, ia, ib);
            retval = false;
        }
    }
    return retval;
}


fn report(b: &[u8], what: &str) -> bool {
    let v = b.iter().rev().into_iter().enumerate().find(|&(idx, val)| {*val != 0}).take();
    match v{
        Some((offset, value)) => {
            println!("For {} found first non zero value {} at offest {}", what, value, b.len() - offset);
            true
        }
        None => {
            println!("For {} found no nonzero value", what);
            false
        }
    }
}

fn get_n_jpegs(fname: String, n: usize) -> Vec<Frame>{
    let (tx, rx) = mpsc::sync_channel(0);
    let file = File::open(fname).expect("Can't even open the inf fifo");
    let dec_t = spawn(||{read_and_send(file, tx)});
    let mut out: Vec<Frame> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Ok(jpeg_buff) = rx.recv() {
            out.push(jpeg_buff);
        }
    }
    return out;
}

fn get_n_frames(fname: String, n: usize) -> Vec<image::RgbImage> {
    let (tx, rx) = mpsc::sync_channel(0);
    let file = File::open(fname).expect("Can't even open the inf fifo");
    let dec_t = spawn(||{read_decode_send(file, tx)});
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        if let Ok(frame) = rx.recv() {
            out.push(frame);
        } else {
            println!("WAT");
        }
    }
    return out;

}

//fn test_lib(jpegs: &Vec<Frame>, marker: &str, work: F)
//    where F: Fn(&Vec<Frame>, &mut Vec<u8>){
fn test_lib(jpegs: &Vec<Frame>, work: &Fn(&Frame)) -> time::Duration
{
    let start = time::Instant::now();
    for j in jpegs {
        work(j);
    }
    start.elapsed()
}

fn test_libs(){
    unsafe{ test(); }
    let jpeg_t_w = |input: &Frame| {
        let mut buffer = vec![0u8; 1920 * 1080 * 3];
        let mut retval = 0;
        unsafe {
            retval = decode_jpeg(input[..].as_ptr(), input.len(), buffer[..].as_ptr());
        }
    };
    let nat_t_w = |input: &Frame| {
        let mut dec = jpeg::Decoder::new(&input[..]);
        dec.decode();
    };

    let nat_decoder = |input: &Frame| {
        let mut dec = jpeg::Decoder::new(&input[..]);
        return dec.decode().expect("Failed to decode with native decoder");
    };

    let jpeg_decoder = |input: &Frame| {
        let mut buffer = vec![0u8; 1920 * 1080 * 3];
        let mut retval = 0;
        unsafe {
            retval = decode_jpeg(input[..].as_ptr(), input.len(), buffer[..].as_ptr());
        };
        return buffer;
    };

    let iters = 90;
    let jpegs = get_n_jpegs("outf".to_string(), iters);

    println!("Starting jpeg_turbo");
    let jt_elapsed = test_lib(&jpegs, &jpeg_t_w);
    println!("jpeg_turbo took {}.{:0>#9} to process {} jpeg frames",
             jt_elapsed.as_secs(),
             jt_elapsed.subsec_nanos(),
             iters);

    println!("Starting native");
    let nat_elapsed = test_lib(&jpegs, &nat_t_w);
    println!("jpeg_turbo took {}.{:0>#9} to process {} jpeg frames",
                nat_elapsed.as_secs(),
                nat_elapsed.subsec_nanos(),
                iters);

    println!("Testing equality");
    for idx in 1..6 {
        let jpeg_dec = jpeg_decoder(&jpegs[5 + idx]);
        let nat_dec = nat_decoder(&jpegs[5 + idx]);
        println!("{} are equal - {}", idx,  are_same(&jpeg_dec, &nat_dec));
    }

    let buffs = get_n_frames("outf".to_string(), iters);
    println!("Testing actual impl");
    for idx in 1..6 {
        let jpeg_dec = jpeg_decoder(&jpegs[5 + idx]);
        println!("{} are equal - {} ", idx, are_same(&jpeg_dec, buffs[5 + idx].clone().into_raw().as_slice()));

    }


}

//fn test_jpeg_turbo(){
//    let work_f = ;
//}

//fn write_to_file()

fn main_old() {

    let mut count = 0;
    let mut s_count = 0;
    let mut e_count = 0;
    let (tx, rx) = mpsc::sync_channel(0);
    let file = File::open("tfile").expect("Can't even open the inf fifo");
    let dec_t = spawn(||{read_and_send(file, tx)});
    loop {
        if let Ok(buff) = rx.recv() {
            let mut outf = File::create(format!("{}.jpg", count)).expect("can't even open up a file");
            outf.write(&buff[..]).expect("Can't even write buff");
            let mut dec = jpeg::Decoder::new(&buff[..]);
            let alt_buf = [0u8; 1920*1080 * 3];
            println!("Len of buff: {}", buff.len());
            let succ = unsafe{ decode_jpeg(buff[..].as_ptr(), buff.len(), alt_buf.as_ptr()) };
            match dec.decode() {
                Ok(decoded) => {
                    count +=1;
                    s_count +=1;
                    println!("{}\t{} - Successfully decoded", count, s_count);
                    println!("libjpeg returned: {}", succ);
                    if succ == 0 {
                        println!("Are they the same?: {}", are_same(&decoded[..], &alt_buf[..]));
                    }
                    report(&decoded[..], "rust_image");
                    report(&alt_buf[..], "libjpg");

                },
                Err(e) => {
                    count +=1;
                    e_count +=1;
                    println!("{}\t{} - Decode err: {}", count, e_count, e);
                }
            }
        } else {
            println!("Failed");
            break;
        }
    }
    dec_t.join().expect("thread failed");
}

fn main() {
    test_libs();
}

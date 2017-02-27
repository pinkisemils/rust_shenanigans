extern crate jpeg_decoder as jpeg;
extern crate image;
extern crate deco;

use deco::{decode_jpeg, read_and_send, Frame, test};

use std::sync::mpsc;
use std::thread::spawn;
use std::fs::File;
use std::time;

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
        let buffer = vec![0u8; 1920 * 1080 * 3];
        unsafe {
            decode_jpeg(input[..].as_ptr(), input.len(), buffer[..].as_ptr());
        }
    };
    let nat_t_w = |input: &Frame| {
        let mut dec = jpeg::Decoder::new(&input[..]);
        dec.decode().expect("All files should decode just fine");
    };

    let nat_decoder = |input: &Frame| {
        let mut dec = jpeg::Decoder::new(&input[..]);
        return dec.decode().expect("Failed to decode with native decoder");
    };

    let jpeg_decoder = |input: &Frame| {
        let buffer = vec![0u8; 1920 * 1080 * 3];
        unsafe {
            decode_jpeg(input[..].as_ptr(), input.len(), buffer[..].as_ptr());
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


}

fn main() {
    test_libs();
}

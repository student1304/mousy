use std::{thread, time};
use rand::{self, Rng};
// use mouse_rs::{types::keys::Keys, Mouse};
use mouse_rs::{Mouse};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// screen size in pixel x_max
    #[arg(default_value_t = 1250)]
    x_max: u16,
    
    /// screen size in pixel y_max
    #[arg(default_value_t = 800)]
    y_max: u16,

    /// time to run in seconds (1h default)
    #[arg(short='s', long, default_value_t = 3600)]
    seconds: u16,
}

fn main() {
    let args = Args::parse();
    println!("size: {}:{}, run for {} seconds", args.x_max, args.y_max, args.seconds);

    for _ in 0..args.seconds {
        let mut rng = rand::thread_rng();
        let x_val = rng.gen_range(1..=1250);
        let y_val =rng.gen_range(1..=800);

        println!("moving mouse to new x, y: {}:{}", x_val, y_val);

        let mouse = Mouse::new();
        mouse.move_to(x_val, y_val).expect("unable to move mouse");
        // mouse.press(&Keys::RIGHT).expect("unable to press button");
        // mouse.release(&Keys::RIGHT).expect("unable to release button");
        

        // asume the above din't take lon
        // we wait 1 sec, so each loop is about 1 sec
        thread::sleep(time::Duration::from_secs(1));
    }

}

//! sierpinksy trinagle image generator
extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;


pub struct Point {
    x: u32,
    y: u32,
}

const WIDTH: u32 = 800 ;
const HEIGHT: u32 = 600;

/// main program
fn main() {
    let mut imag = image::ImageBuffer::from_fn(WIDTH,Height,|x,y| {
        if x == 0 && y == 0 {
            image::Luma([ou8])
        }else{
            image::Luma([255u8])
        }
    });

    let mut cnt: u32 = 10_000;
    let pts:

}

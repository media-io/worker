
extern crate worker;

use worker::mediaio::plugin::*;

fn main() {
  let plugin = search("ffmpeg");
  println!("{:?}", plugin);
}

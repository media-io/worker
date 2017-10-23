
extern crate worker;

use worker::mediaio::plugin::*;

fn main() {
  let plugins = get_all_plugins();
  for plugin in plugins {
    println!("{}", plugin.identifier);
  }

}

mod config;
use config::RandomizerConfig;

mod rando_treasure;
use rando_treasure::rando_treasure;

mod helper_functions;
use helper_functions::*;

use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let region = &args[1].to_lowercase();
    let path = &args[2];

    let treasure_offset_start: usize = match region.as_str() {
        "eu" => 0x7E117C18,
        "us" => 0x8AAD7C18,
        _ => panic!(""),
    };

    let treasure_offset_end: usize = match region.as_str() {
        "eu" => 0x7E122BED,
        "us" => 0x8AAE2BED,
        _ => panic!(),
    };

    let treasure_offset_diff = treasure_offset_end - treasure_offset_start + 1;

    let config = RandomizerConfig {
        seed: None,
        p_treasure: 0.55,
        p_treasure_item: 0.5,
    };

    let treasure_data = read_file(path, treasure_offset_start as u64, treasure_offset_diff);
  
    let r_treasure = loop {
        let mut r_treasure = rando_treasure(treasure_data.clone(), &config);
        if r_treasure.len() <= treasure_offset_diff {
            r_treasure.extend(vec![0; treasure_offset_diff - r_treasure.len()]);
            break r_treasure;
        }
    };

    write_file(path, r_treasure, treasure_offset_start as u64);
}

use crate::config::RandomizerConfig;
use crate::helper_functions::*;

use rand::{thread_rng, Rng};

pub fn rando_treasure(data: Vec<u8>, config: &RandomizerConfig) -> Vec<u8> {
    let treasures = data.split(|x| *x == 0x0A).map(|x| x.to_owned());

    let result = treasures
        .map(|t| process_treasure(t, config))
        .collect::<Vec<_>>();

    result.join(&0x0A)
}

fn process_treasure(data: Vec<u8>, config: &RandomizerConfig) -> Vec<u8> {
    let mut params = data
        .split(|x| *x == 0x2C)
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    if config.is_treasure() {
        if config.is_treasure_item() {
            params[1] = int2bytes_str(0);
            params[2] = int2bytes_str(thread_rng().gen_range(1..=388));
        } else {
            params[1] = int2bytes_str(1);
            params[2] = int2bytes_str(thread_rng().gen_range(1..=999));
        }
    } else {
        params[1] = int2bytes_str(0);
        params[2] = int2bytes_str(-1);
    }

    params.join(&0x2C)
}

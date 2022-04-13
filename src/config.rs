use rand::{thread_rng, Rng};

pub struct RandomizerConfig {
    pub seed: Option<u32>,
    pub p_treasure: f32,
    pub p_treasure_item: f32,
}

impl RandomizerConfig {
    pub fn is_treasure(&self) -> bool {
        thread_rng().gen::<f32>() < self.p_treasure
    }

    pub fn is_treasure_item(&self) -> bool {
        thread_rng().gen::<f32>() < self.p_treasure_item
    }
}

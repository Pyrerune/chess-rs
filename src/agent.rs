use std::collections::HashMap;
use std::fs::File;

#[derive(Clone, Debug)]
pub struct Agent {
    states: Vec<String>,
    lr: f32,
    exp_rate: f32,
    decay_gamma: f32,
    states_value: HashMap<String, f32>,
    name: String,
}
impl Agent {
    pub fn new(name: String, exp_rate: Option<f32>) -> Agent {
        Agent {
            states: vec![],
            lr: 0.2,
            exp_rate: exp_rate.unwrap_or(0.3),
            decay_gamma: 0.9,
            states_value: HashMap::new(),
            name,
        }

    }
    pub fn save(&self, filename: String) {
        let file = File::create(filename).expect("Could not create policy");

        serde_json::to_writer(file, &self.states_value);
    }
    fn try_load(&mut self, filename: String) {
        let file = File::open(filename);
        if file.is_err() {
            eprintln!("Failed to load agent");
            return;
        }
        self.states_value = serde_json::from_reader(file.unwrap()).unwrap();
    }
}

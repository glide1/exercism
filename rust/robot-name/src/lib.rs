#[macro_use]
extern crate lazy_static;

extern crate rand;

use std::sync::Mutex;
use std::collections::HashSet;
use rand::distributions::IndependentSample;

lazy_static! {
    static ref ROBOT_FACTORY: Mutex<RobotFactory> =
        Mutex::new(RobotFactory::new());
}

struct RobotFactory {
    names: HashSet<String>,
    character_range: rand::distributions::Range<u8>,
    number_range: rand::distributions::Range<u8>,
}

pub struct Robot {
    name: String,
}

impl RobotFactory {
    fn new() -> Self {
        RobotFactory {
            names: HashSet::new(),
            character_range: rand::distributions::Range::new(b'A', b'Z' + 1),
            number_range: rand::distributions::Range::new(b'0', b'9' + 1),
        }
    }

    fn create_robot<'a>(&mut self) -> Robot {
        Robot { name: self.create_name() }
    }

    fn create_name(&mut self) -> String {
        let mut rng = rand::thread_rng();

        let mut name = String::from("");
        while {
                  name.clear();
                  name.push(self.character_range.ind_sample(&mut rng) as char);
                  name.push(self.character_range.ind_sample(&mut rng) as char);
                  name.push(self.number_range.ind_sample(&mut rng) as char);
                  name.push(self.number_range.ind_sample(&mut rng) as char);
                  name.push(self.number_range.ind_sample(&mut rng) as char);
                  !self.names.insert(name.clone())
              } {}
        name
    }
}

impl Robot {
    pub fn new() -> Self {
        ROBOT_FACTORY.lock().unwrap().create_robot()
    }

    pub fn name<'a>(&'a self) -> &'a str {
        self.name.as_ref()
    }
    pub fn reset_name(&mut self) {
        self.name = ROBOT_FACTORY.lock().unwrap().create_name()
    }
}


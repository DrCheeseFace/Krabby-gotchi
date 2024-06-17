use rand::{thread_rng, Rng};

#[derive(Debug, Savefile)]
pub struct Krab {
    name: String,
    hunger: u16,
    happiness: u16,
    health: u16,
    age: u64,
    mood: String,
    status: String,
    stage: String,
}

impl Krab {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hunger: 0,
            happiness: 500,
            health: 500,
            age: 0,
            mood: String::from("neutral"),
            status: String::from("alive"),
            stage: String::from("egg"),
        }
    }
    pub fn grow_older(&mut self) {
        //TODO vary += x based on current stats
        self.age += 1;
        match self.age {
            0..=500 => {
                self.stage = String::from("egg");
            }
            501..=6600 => {
                self.stage = String::from("baby");
            }
            6601..=150700 => {
                self.stage = String::from("teen");
            }
            150701..=582800 => {
                self.stage = String::from("adult");
            }
            _ => {
                self.stage = String::from("elder");
            }
        }
    }
    pub fn on_tick(&mut self) {
        // I FUCKING LOVE IF ELSE
        if self.status != "dead" {
            self.grow_older();

            if self.hunger == 0 {
                if self.health == 0 {
                    self.status = String::from("dead");
                } else {
                    self.injure()
                }
            }

            if self.hunger > 800 {
                self.heal();
                self.pet();
            }
            if self.hunger < 500 {
                self.injure();
            }

            if (self.hunger < 300) == (self.health < 300) {
                self.sadder(5);
            } else {
                self.sadder(2);
            }
            if self.happiness < 300 {
                self.starve()
            }

            let mut rng = thread_rng();
            let m: f64 = (self.hunger as f64 + self.health as f64) / 2000 as f64;
            let is_happy: bool = rng.gen_bool(m);
            if is_happy {
                self.happier(2);
            } else {
                self.sadder(2);
            }
        } else {
            self.happiness = 0;
            self.hunger = 0;
        }
        self.starve();
    }

    pub fn pet(&mut self) {
        if self.happiness < 995 {
            self.happiness += 5;
        } else {
            self.happiness = 1000;
        }
    }

    pub fn sadder(&mut self, n: u16) {
        if self.happiness > n {
            self.happiness -= n;
        } else {
            self.happiness = 0;
        }
    }
    pub fn happier(&mut self, n: u16) {
        if self.happiness < 1000 - n {
            self.happiness += n;
        } else {
            self.happiness = 1000;
        }
    }

    pub fn heal(&mut self) {
        if self.health < 995 {
            self.health += 5;
        } else {
            self.health = 1000;
        }
    }

    pub fn injure(&mut self) {
        if self.health > 1 {
            self.health -= 1;
        } else {
            self.health = 0;
        }
    }
    pub fn starve(&mut self) {
        if self.hunger > 1 {
            self.hunger -= 1;
        } else {
            self.hunger = 0;
        }
    }

    pub fn feed(&mut self) {
        if self.hunger < 995 {
            self.hunger += 5;
        } else {
            self.hunger = 1000;
        }
    }

    //getters
    #[warn(dead_code)]
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn age(&self) -> &u64 {
        &self.age
    }
    pub fn hunger(&self) -> &u16 {
        &self.hunger
    }
    pub fn happiness(&self) -> &u16 {
        &self.happiness
    }
    pub fn health(&self) -> &u16 {
        &self.health
    }
    pub fn status(&self) -> &String {
        &self.status
    }
    pub fn stage(&self) -> &String {
        &self.stage
    }
}

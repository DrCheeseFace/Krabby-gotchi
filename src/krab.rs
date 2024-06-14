#[derive(Debug, Savefile)]
pub struct Krab {
    name: String,
    hunger: u16,
    happiness: u16,
    health: u16,
    age: u64,
    weight: u16,
    size: u16,
    mood: String,
    status: String,
    stage: String,
}

impl Krab {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hunger: 0,
            happiness: 50,
            health: 100,
            age: 0,
            weight: 0,
            size: 0,
            mood: String::from("neutral"),
            status: String::from("alive"),
            stage: String::from("egg"),
        }
    }
    pub fn grow_older(&mut self) {
        //TODO vary += x based on current stats
        self.age += 1;
        match self.age {
            0..=5 => {
                self.stage = String::from("egg");
            }
            6..=66 => {
                self.stage = String::from("baby");
            }
            67..=1507 => {
                self.stage = String::from("teen");
            }
            1508..=5828 => {
                self.stage = String::from("adult");
            }
            _ => {
                self.stage = String::from("elder");
            }
        }
    }

    pub fn pet(&mut self) {
        if self.happiness < 95 {
            self.happiness += 5;
        } else {
            self.happiness = 100;
        }
    }

    pub fn feed(&mut self) {
        if self.hunger < 95 {
            self.hunger += 5;
        } else {
            self.hunger = 100;
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
    pub fn weight(&self) -> &u16 {
        &self.weight
    }
    pub fn mood(&self) -> &String {
        &self.mood
    }
    // pub fn size(&self) -> &u16 {
    //     &self.size
    // }
    // pub fn status(&self) -> &String {
    //     &self.status
    // }
    // pub fn stage(&self) -> &String {
    //     &self.stage
    // }
}

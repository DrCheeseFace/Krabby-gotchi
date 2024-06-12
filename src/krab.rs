#[derive(Debug, Savefile)]
pub struct Krab {
    name: String,
    hunger: u8,
    happiness: u8,
    health: u8,
    age: u64,
    weight: u8,
    size: u8,
    mood: String,
    status: String,
    stage: String,
}

impl Krab {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hunger: 0,
            happiness: 0,
            health: 0,
            age: 0,
            weight: 0,
            size: 0,
            mood: String::from("neutral"),
            status: String::from("alive"),
            stage: String::from("egg"),
        }
    }
    pub fn grow_older(&mut self) {
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
    
    pub fn feed(&mut self) {
        todo!()
    }


    //getters
    #[warn(dead_code)]
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn age(&self) -> &u64 {
        &self.age
    }

    // pub fn size(&self) -> &u8 {
    //     &self.size
    // }
    // pub fn hunger(&self) -> &u8 {
    //     &self.hunger
    // }
    // pub fn happiness(&self) -> &u8 {
    //     &self.happiness
    // }
    // pub fn health(&self) -> &u8 {
    //     &self.health
    // }
    // pub fn weight(&self) -> &u8 {
    //     &self.weight
    // }
    // pub fn mood(&self) -> &String {
    //     &self.mood
    // }
    // pub fn status(&self) -> &String {
    //     &self.status
    // }
    // pub fn stage(&self) -> &String {
    //     &self.stage
    // }
    
}

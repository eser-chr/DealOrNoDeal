use std::array;
use rand::seq::SliceRandom;
pub struct Case {
    pub money: f32,
    pub is_in: bool,
}

impl Case {
    pub fn new(money: f32) -> Self {
        Self { money, is_in: true }
    }
}


pub enum RemovalMode {
    Silent,
    Announce,
}

pub struct Cases {
    pub cases: [Case; 26],
}

impl Cases {
    pub fn new() -> Self {
        let mut values: [f32; 26] = [
            0.01, 0.2, 0.5, 1.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0, 300.0, 400.0, 500.0, 1000.0,
            2500.0, 5000.0, 7500.0, 10000.0, 15000.0, 25000.0, 50000.0, 75000.0, 100000.0,
            200000.0, 300000.0, 500000.0,
        ];

        let mut rng = rand::rng();
        values.shuffle(&mut rng);
        let cases: [Case; 26] = array::from_fn(|i| Case::new(values[i]));
        Self { cases }
    }

    pub fn print_valid(&self) {
        for (i, case) in self.cases.iter().enumerate() {
            if case.is_in {
                print!("{} ", i + 1);
            }
        }
    }

    pub fn remove_case(&mut self, case_idx: u8, show_case_content: RemovalMode) {
        let val = self.cases[case_idx as usize].money;
        self.cases[case_idx as usize].is_in = false;
        if let show_case_content = RemovalMode::Announce{
            
        }
        match show_case_content {
            RemovalMode::Announce => {
                println!(
                    "You opened case {}. It contained 💸 {} 💰 \n",
                    case_idx + 1,
                    val
                );
            }
            _ => {}
        }
    }

    pub fn is_valid(&self, index: u8) -> bool {
        let idx = (index - 1) as usize;
        idx < self.cases.len() && self.cases[idx].is_in
    }

    pub fn mean(&self) -> f32 {
        let sum: f32 = self.cases.iter().filter(|c| c.is_in).map(|c| c.money).sum();
        sum / (self.get_len_of_valid() as f32)
    }

    pub fn get_len_of_valid(&self) -> usize {
        self.cases.iter().filter(|c| c.is_in).count()
    }
}
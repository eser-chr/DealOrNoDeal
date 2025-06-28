use rand::seq::SliceRandom;
use std::array;

mod helpers {
    use std::io::{self, Write};
    pub fn oi(msg: &str, buf: &mut String) {
        println!("{msg}");
        io::stdout().flush().unwrap();
        io::stdin().read_line(buf).unwrap();
        println!("\n ###########\n")
    }

    // pub fn oi_case_
}

struct Player {
    name: String,
    case: u8,
}

impl Player {
    fn from_cli() -> Self {
        let mut buf = String::new();
        helpers::oi("Welcome to Super Deal! Please write your name: ", &mut buf);
        let name = buf.trim().to_string();
        Self { name, case: 0 }
    }

    fn pick_case(&mut self, cases: &mut Cases) {
        println!("So are you ready to play {}?", self.name);

        let case: u8 = loop {
            let mut buf = String::new();
            helpers::oi("Please enter a number from 1-26!", &mut buf);

            match buf.trim().parse::<u8>() {
                Ok(val) if (1..=26).contains(&val) => break val,
                _ => {
                    continue;
                }
            }
        };

        self.case = case;
        cases.remove_case(case);
    }

    fn deal_handle(&self, offer: f32) -> bool {
        print!("I offer you {} E. Do we have a deal?", offer);
        let is_deal: bool = loop {
            let mut buf = String::new();
            helpers::oi(" [y/n] ", &mut buf);

            match buf.trim() {
                "y" => break true,
                "n" => break false,
                _ => {
                    continue;
                }
            }
        };
        is_deal
    }

    fn remove_case(&self, cases: &mut Cases) {
        println!("Choose a bag to remove: ");
        cases.print_valid();

        let case: u8 = loop {
            let mut buf = String::new();
            helpers::oi("Please enter a number from 1-26!", &mut buf);

            match buf.trim().parse::<u8>() {
                Ok(val) if (1..=26).contains(&val) => break val,
                _ => {
                    continue;
                }
            }
        };

        cases.remove_case(case);
    }
}

struct Case {
    money: f32,
    is_in: bool,
}

impl Case {
    fn new(money: f32) -> Self {
        Self { money, is_in: true }
    }
}

struct Cases {
    cases: [Case; 26],
    valid_indices: Vec<u8>,
}

impl Cases {
    fn new() -> Self {
        let mut values: [f32; 26] = [
            0.01, 0.2, 0.5, 1.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0, 300.0, 400.0, 500.0, 1000.0,
            2500.0, 5000.0, 7500.0, 10000.0, 15000.0, 25000.0, 50000.0, 75000.0, 100000.0,
            200000.0, 300000.0, 500000.0,
        ];

        let mut rng = rand::rng();
        values.shuffle(&mut rng);

        let cases: [Case; 26] = array::from_fn(|i| Case::new(values[i]));
        let valid_indices: Vec<u8> = (1..=26).collect();
        Self {
            cases,
            valid_indices,
        }
    }

    fn print_valid(&self) {
        for i in self.valid_indices.iter() {
            print!("{i} ");
        }
    }

    fn remove_case(&mut self, case_idx: u8) {
        self.valid_indices.retain(|x| *x != case_idx);
        self.cases[case_idx as usize].is_in = false;
    }

    fn get_mean(&self) -> f32 {
        let mut sum = 0.0;
        for case in &self.cases {
            if case.is_in {
                sum += case.money;
            }
        }

        sum / (self.valid_indices.len() as f32)
    }
}

struct Banker;
impl Banker {
    fn make_offer(&self, cases: &Cases) -> f32 {
        0.4 * cases.get_mean()
        // 10.0
    }
}

enum Action {
    PickCase,
    RemoveCase,
    BankOffer,
    OfferDecision(f32),
    OfferAccept,
    OfferDeny,
    RoundEnded,
    End,
}

fn decide_next_action(round: usize) -> Action {
    if round % 4 == 0 {
        Action::BankOffer
    } else {
        Action::RemoveCase
    }
}

fn main() {
    let mut player = Player::from_cli();
    let mut cases = Cases::new();
    let banker: Banker = Banker {};
    // player.pick_case(& mut cases);

    let mut round: usize = 0;
    let mut action: Action = Action::PickCase;

    loop {
        action = match action {
            Action::PickCase => {
                player.pick_case(&mut cases);
                Action::RoundEnded
            }
            Action::RoundEnded => {
                round += 1;
                decide_next_action(round)
            }
            Action::RemoveCase => {
                player.remove_case(&mut cases);
                Action::RoundEnded
            }

            Action::BankOffer => {
                let offer = banker.make_offer(&cases);
                Action::OfferDecision(offer)
            }
            Action::OfferDecision(val) => {
                let is_deal = player.deal_handle(val);
                if is_deal {
                    Action::OfferAccept
                } else {
                    Action::OfferDeny
                }
            }
            Action::OfferAccept => Action::End,
            Action::OfferDeny => Action::RemoveCase,
            Action::End => {
                println!("You won !");
                break;
            }
        };
    }
}

use colored::*;
use rand::seq::SliceRandom;
use std::array;

const NUM_CASES: u8 = 26;

mod helpers {
    use crate::Cases;
    use colored::*;
    use std::io::{self, Write};

    pub fn format_banner(title: &str) -> String {
        format!(
            "{}\n{}\n{}",
            "=".repeat(40),
            title.bold().cyan(),
            "=".repeat(40)
        )
    }

    pub fn prompt_oi(msg: &str, buf: &mut String) {
        println!("{msg}");
        io::stdout().flush().unwrap();
        io::stdin().read_line(buf).unwrap();
        clear_screen();
        println!("\n {}\n", "=".repeat(40));
    }

    pub fn read_case_number(cases: &Cases) -> u8 {
        loop {
            let mut buf = String::new();
            prompt_oi("Please enter a number from 1-26!", &mut buf);

            match buf.trim().parse::<u8>() {
                Ok(val) => {
                    if (1..=super::NUM_CASES).contains(&val) && cases.is_valid(val) {
                        break val;
                    } else if (1..=super::NUM_CASES).contains(&val) {
                        println!(
                            "{}",
                            "This number is already out of the game!".yellow().bold()
                        );
                    } else {
                        println!("{}", "Not valid number!".red().bold());
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }

    pub fn read_yes_no() -> bool {
        loop {
            let mut buf = String::new();
            prompt_oi(&format!("{}", " [y/n] ".bold()), &mut buf);

            match buf.trim().to_lowercase().as_str() {
                "y" => break true,
                "yes" => break true,

                "n" => break false,
                "no" => break false,

                _ => {
                    continue;
                }
            }
        }
    }

    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }
}

struct Player {
    name: String,
    case: u8,
}

impl Player {
    fn from_cli() -> Self {
        let mut buf = String::new();
        helpers::prompt_oi(
            &format!(
                "{}",
                helpers::format_banner(
                    &"🎉 !!! Welcome to SUPER DEAL !!! 🎉 \n Please write your name: "
                        .bold()
                        .red()
                )
            ),
            &mut buf,
        );
        let name = buf.trim().to_string();
        Self { name, case: 0 }
    }

    fn pick_case(&mut self, cases: &mut Cases) {
        println!(
            "{}",
            format!(
                "So... Are you ready to play {} and win HALF a MILLION?",
                self.name.to_uppercase().bold()
            )
        );
        println!("{}", "First, you have to pick your case!".bold());
        let case = helpers::read_case_number(cases);
        self.case = case;
        cases.remove_case(case - 1, RemovalMode::Silent);
    }

    fn remove_case(&self, cases: &mut Cases) {
        println!("{}", "Choose a case 💼 to remove: ".underline());
        println!("{}", "💼 Remaining Cases 💼".bold());

        cases.print_valid();
        let case = helpers::read_case_number(cases);
        cases.remove_case(case - 1, RemovalMode::Announce);
    }

    fn deal_handle(&self, offer: f32) -> bool {
        print!("\n💰 BANKER'S OFFER: {:.2} €. Deal?", offer);
        helpers::read_yes_no()
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

enum RemovalMode {
    Silent,
    Announce,
}
struct Cases {
    cases: [Case; 26],
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
        Self { cases }
    }

    fn print_valid(&self) {
        for (i, case) in self.cases.iter().enumerate() {
            if case.is_in {
                print!("{} ", i + 1);
            }
        }
    }

    fn remove_case(&mut self, case_idx: u8, show_case_content: RemovalMode) {
        let val = self.cases[case_idx as usize].money;
        self.cases[case_idx as usize].is_in = false;
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

    fn is_valid(&self, index: u8) -> bool {
        let idx = (index - 1) as usize;
        idx < self.cases.len() && self.cases[idx].is_in
    }

    fn mean(&self) -> f32 {
        let sum: f32 = self.cases.iter().filter(|c| c.is_in).map(|c| c.money).sum();
        sum / (self.get_len_of_valid() as f32)
    }

    fn get_len_of_valid(&self) -> usize {
        self.cases.iter().filter(|c| c.is_in).count()
    }
}

struct Banker;
impl Banker {
    fn make_offer(&self, cases: &Cases) -> f32 {
        0.4 * cases.mean()
    }
}

enum Action {
    PickCase,
    RemoveCase,
    BankOffer,
    OfferDecision(f32),
    OfferAccept(f32),
    OfferDeny,
    RoundEnded,
    End(f32),
}

fn main() {
    let mut player = Player::from_cli();
    let mut cases = Cases::new();
    let banker: Banker = Banker {};

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

                if cases.get_len_of_valid() == 0 {
                    Action::End(cases.cases[(player.case - 1) as usize].money)
                } else if round % 4 == 0 {
                    Action::BankOffer
                } else {
                    Action::RemoveCase
                }

                // decide_next_action(round)
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
                    Action::OfferAccept(val)
                } else {
                    Action::OfferDeny
                }
            }
            Action::OfferAccept(val) => Action::End(val),
            Action::OfferDeny => Action::RemoveCase,
            Action::End(val) => {
                println!("{}", "Well, done!".bold());
                println!("You won {} 💰!", val);
                break;
            }
        };
    }
}

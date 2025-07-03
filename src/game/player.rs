use colored::*;
use super::helpers;
use super::cases::{Cases, RemovalMode};
pub struct Player {
    pub name: String,
    pub case: u8,
}

impl Player {
    pub fn from_cli() -> Self {
        let mut buf = String::new();
        helpers::prompt_oi(
            &helpers::format_banner(
                &"🎉 !!! Welcome to SUPER DEAL !!! 🎉 \n Please write your name: "
                    .bold()
                    .red(),
            ),
            &mut buf,
        );
        let name = buf.trim().to_string();
        Self { name, case: 0 }
    }

    pub fn pick_case(&mut self, cases: &mut Cases) {
        println!(
            "So... Are you ready to play {} and win HALF a MILLION?",
            self.name.to_uppercase().bold()
        );
        println!("{}", "First, you have to pick your case!".bold());
        let case = helpers::read_case_number(cases);
        self.case = case;
        cases.remove_case(case - 1, RemovalMode::Silent);
    }

    pub fn remove_case(&self, cases: &mut Cases) {
        println!("{}", "Choose a case 💼 to remove: ".underline());
        println!("{}", "💼 Remaining Cases 💼".bold());

        cases.print_valid();
        let case = helpers::read_case_number(cases);
        cases.remove_case(case - 1, RemovalMode::Announce);
    }

    pub fn deal_handle(&self, offer: f32) -> bool {
        print!("\n💰 BANKER'S OFFER: {:.2} €. Deal?", offer);
        helpers::read_yes_no()
    }
}
use super::banker::Banker;
use super::player::Player;
use super::cases::Cases;
use colored::*;
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

pub fn run_game() {
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

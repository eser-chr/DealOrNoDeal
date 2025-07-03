
use super::cases::Cases;

pub struct Banker;
impl Banker {
    pub fn make_offer(&self, cases: &Cases) -> f32 {
        0.8 * cases.mean()
    }
}
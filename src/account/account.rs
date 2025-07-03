pub struct User {
    name: String,
    password: String,
    avg_money_won: f64,
    games_played: f64,
}

impl User {
    fn from_cli() {}

    fn login(){}

    fn update_score(&mut self, money_won: f64) {
        let money_won = self.games_played * self.avg_money_won + money_won;
        self.games_played+=1;
        self.avg_money_won = money_won / self.games_played;
    }

    fn save(&self){}
}

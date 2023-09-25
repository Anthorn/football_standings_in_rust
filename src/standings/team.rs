
#[derive(Default, Debug)]
pub struct TeamStats {
    pub name: String,
    pub played: u8,
    pub wins: u8,
    pub draws: u8,
    pub defeats: u8,
    pub goal_scored: i32,
    pub goal_against: i32,
    pub goal_difference: i32,
    pub points: u8,
}

impl TeamStats {
    pub fn calc_stats(&mut self) {
        self.played = self.wins + self.draws + self.defeats;
        self.goal_difference = self.goal_scored - self.goal_against;
        self.points = self.wins * 3 + self.draws;
    }

    fn update_goals(&mut self, goal_scored: i32, goal_conceded: i32) {
        self.goal_scored += goal_scored;
        self.goal_against += goal_conceded;
    }

    pub fn add_win(&mut self, goal_scored: i32, goal_conceded: i32) {
        self.wins += 1;
        self.update_goals(goal_scored, goal_conceded);
    }

    pub fn add_draw(&mut self, goal_scored: i32, goal_conceded: i32) {
        self.draws += 1;
        self.update_goals(goal_scored, goal_conceded);
    }

    pub fn add_defeat(&mut self, goal_scored: i32, goal_conceded: i32) {
        self.defeats += 1;
        self.update_goals(goal_scored, goal_conceded);
    }
}

pub fn create_team_template() -> TeamStats {
    TeamStats {
        name: String::from(""),
        played: 0,
        wins: 0,
        draws: 0,
        defeats: 0,
        goal_scored: 0,
        goal_against: 0,
        goal_difference: 0,
        points: 0,
    }
}

pub fn create_team(team_str: &String) -> Result<TeamStats, &'static str> {
    let parts: Vec<&str> = team_str.split(";").collect();

    if parts.len() != 7 {
        Err("Team line contains more than seven(7) parts. Please check your table file.")
    } else {
        let team_name = parts[0].trim();

        let played = parts[1]
            .trim()
            .parse::<u8>()
            .map_err(|_| "Could not parse number of played games.")?;
        let wins = parts[2]
            .trim()
            .parse::<u8>()
            .map_err(|_| "Could not parse number of wins.")?;
        let draws = parts[3]
            .trim()
            .parse::<u8>()
            .map_err(|_| "Could not parse number of draws.")?;
        let defeats = parts[4]
            .trim()
            .parse::<u8>()
            .map_err(|_| "Could not parse number of defeats.")?;
        let goal_scored = parts[5]
            .trim()
            .parse::<i32>()
            .map_err(|_| "Could not parse number of goals scored.")?;
        let goal_conceded = parts[6]
            .trim()
            .parse::<i32>()
            .map_err(|_| "Could not parse number of goals conceded.")?;
        Ok(TeamStats {
            name: team_name.to_string(),
            played: played,
            wins: wins,
            draws: draws,
            defeats: defeats,
            goal_scored: goal_scored,
            goal_against: goal_conceded,
            ..create_team_template()
        })
    }
}

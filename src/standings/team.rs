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
        Err("Team line does not contain exactly seven(7) parts. Please check your table file.")
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
            .map_err(|_| "Could not parse number of scored goals.")?;
        let goal_conceded = parts[6]
            .trim()
            .parse::<i32>()
            .map_err(|_| "Could not parse number of conceded goals.")?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_team_template() {
        let template = create_team_template();
        assert_eq!(template.name, String::from(""));
        assert_eq!(template.played, 0);
        assert_eq!(template.wins, 0);
        assert_eq!(template.draws, 0);
        assert_eq!(template.defeats, 0);
        assert_eq!(template.points, 0);
        assert_eq!(template.goal_scored, 0);
        assert_eq!(template.goal_against, 0);
        assert_eq!(template.goal_difference, 0);
    }

    #[test]
    fn test_update_win_team() {
        let mut template = create_team_template();
        template.add_win(2, 1);
        template.calc_stats();

        assert_eq!(template.name, String::from(""));
        assert_eq!(template.played, 1);
        assert_eq!(template.wins, 1);
        assert_eq!(template.draws, 0);
        assert_eq!(template.defeats, 0);
        assert_eq!(template.points, 3);
        assert_eq!(template.goal_scored, 2);
        assert_eq!(template.goal_against, 1);
        assert_eq!(template.goal_difference, 1);
    }

    #[test]
    fn test_update_draw_team() {
        let mut template = create_team_template();
        template.add_draw(2, 2);
        template.calc_stats();

        assert_eq!(template.name, String::from(""));
        assert_eq!(template.played, 1);
        assert_eq!(template.wins, 0);
        assert_eq!(template.draws, 1);
        assert_eq!(template.defeats, 0);
        assert_eq!(template.points, 1);
        assert_eq!(template.goal_scored, 2);
        assert_eq!(template.goal_against, 2);
        assert_eq!(template.goal_difference, 0);
    }

    #[test]
    fn test_update_defeat_team() {
        let mut template = create_team_template();
        template.add_defeat(1, 2);
        template.calc_stats();

        assert_eq!(template.name, String::from(""));
        assert_eq!(template.played, 1);
        assert_eq!(template.wins, 0);
        assert_eq!(template.draws, 0);
        assert_eq!(template.defeats, 1);
        assert_eq!(template.points, 0);
        assert_eq!(template.goal_scored, 1);
        assert_eq!(template.goal_against, 2);
        assert_eq!(template.goal_difference, -1);
    }

    #[test]
    fn test_create_team() {
        let input_team_raw = String::from("IFK Göteborg;10;5;3;2;10;2");

        let mut parsed_team = create_team(&input_team_raw).unwrap();
        parsed_team.calc_stats();

        assert_eq!(parsed_team.name, String::from("IFK Göteborg"));
        assert_eq!(parsed_team.played, 10);
        assert_eq!(parsed_team.wins, 5);
        assert_eq!(parsed_team.draws, 3);
        assert_eq!(parsed_team.defeats, 2);
        assert_eq!(parsed_team.points, 18);
        assert_eq!(parsed_team.goal_scored, 10);
        assert_eq!(parsed_team.goal_against, 2);
        assert_eq!(parsed_team.goal_difference, 8);
    }

    #[test]
    #[should_panic(expected = "Could not parse number of played games.")]
    fn test_create_team_panic_incorrect_played() {
        let input_team_raw = String::from("IFK Göteborg;a;5;3;2;10;2");

        let _parsed_team = create_team(&input_team_raw).unwrap();
    }

    #[test]
    #[should_panic(expected = "Could not parse number of wins.")]
    fn test_create_team_panic_incorrect_wins() {
        let input_team_raw = String::from("IFK Göteborg;10;-2;3;2;10;2");

        let _parsed_team = create_team(&input_team_raw).unwrap();
    }

    #[test]
    #[should_panic(expected = "Could not parse number of draws.")]
    fn test_create_team_panic_incorrect_draws() {
        let input_team_raw = String::from("IFK Göteborg;10;2;-3;2;10;2");

        let _parsed_team = create_team(&input_team_raw).unwrap();
    }

    #[test]
    #[should_panic(expected = "Could not parse number of defeats.")]
    fn test_create_team_panic_incorrect_defeats() {
        let input_team_raw = String::from("IFK Göteborg;10;2;3;-2;10;2");

        let _parsed_team = create_team(&input_team_raw).unwrap();
    }
    #[test]
    #[should_panic(expected = "Could not parse number of scored goals.")]
    fn test_create_team_panic_incorrect_scored_goals() {
        let input_team_raw = String::from("IFK Göteborg;10;2;3;2;!;2");

        let _parsed_team = create_team(&input_team_raw).unwrap();
    }

    #[test]
    #[should_panic(expected = "Could not parse number of conceded goals.")]
    fn test_create_team_panic_incorrect_conceded_goals() {
        let input_team_raw = String::from("IFK Göteborg;10;2;3;2;10;Ä");

        let _parsed_team = create_team(&input_team_raw).unwrap();
    }
}

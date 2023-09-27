use crate::standings::team;
use crate::standings::team::TeamStats;

pub struct Table {
    pub teams: Vec<TeamStats>,
}
impl Table {
    pub fn print(&mut self) {
        println!("--------- Allsvenskan ------------");

        let column_width: usize = self
            .teams
            .iter()
            .map(|row| row.name.len())
            .max()
            .unwrap_or(0);

        println!(
            "{:<width1$} {:<width2$}  {:<width2$}  {:<width2$}  {:<width2$}  {:<width2$}  {:<width2$}  {:<width3$}  {:<width2$}",
            "Club",
            "G",
            "W",
            "D",
            "L",
            "GM",
            "GC",
            "GS",
            "P",
            width1 = column_width,
            width2 = 2,
            width3 = 3
        );

        for team in &mut self.teams {
            println!(
                "{:<width1$} {:<width2$}  {:<width2$}  {:<width2$}  {:<width2$}  {:<width2$}  {:<width2$}  {:<width3$}  {:<width2$}",
                team.name.clone(),
                &team.played,
                &team.wins,
                &team.draws,
                &team.defeats,
                &team.goal_scored,
                &team.goal_against,
                &team.goal_difference,
                &team.points,
                width1 = column_width,
                width2 = 2,
                width3 = 3
            );
        }
    }

    pub fn update_table(&mut self) {
        for team in &mut self.teams {
            team.calc_stats();
        }

        self.teams.sort_by(|team1, team2| {
            if team1.points != team2.points {
                team2.points.cmp(&team1.points)
            } else {
                team2.goal_difference.cmp(&team1.goal_difference)
            }
        });
    }

    pub fn add_game(&mut self, home_team: &str, away_team: &str, home_score: i32, away_score: i32) {
        {
            let result: Option<&mut TeamStats> =
                self.teams.iter_mut().find(|team| team.name == home_team);

            match result {
                Some(home_team) => {
                    if home_score > away_score {
                        home_team.add_win(home_score, away_score);
                    } else if home_score < away_score {
                        home_team.add_defeat(home_score, away_score);
                    } else {
                        home_team.add_draw(home_score, away_score);
                    }
                }
                None => {
                    println!("Home team not found...");
                    return;
                }
            }
        }
        {
            let result: Option<&mut TeamStats> =
                self.teams.iter_mut().find(|team| team.name == away_team);
            match result {
                Some(away_team) => {
                    if away_score > home_score {
                        away_team.add_win(away_score, home_score);
                    } else if away_score < home_score {
                        away_team.add_defeat(away_score, home_score);
                    } else {
                        away_team.add_draw(away_score, home_score);
                    }
                }
                None => {
                    println!("Away team not found...");
                    return;
                }
            }
        }
    }
}

pub fn create_table(teams: &[String]) -> Table {
    let team_template = team::create_team_template();
    let mut current_teams: Vec<TeamStats> = vec![];
    for team in teams {
        let current_team = TeamStats {
            name: team.clone(),
            ..team_template
        };
        current_teams.push(current_team);
    }

    Table {
        teams: current_teams,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table() {
        let team_names = vec![
            String::from("FC Test"),
            String::from("Foo"),
            String::from("Bar"),
        ];

        let test_table = create_table(&team_names);
        assert_eq!(test_table.teams.len(), 3);

        test_table
            .teams
            .into_iter()
            .enumerate()
            .for_each(|(i, team)| assert_eq!(team_names[i], team.name));
    }

    #[test]
    fn test_add_game_home_team_win() {
        let team_names = vec![
            String::from("FC Test"),
            String::from("Foo"),
            String::from("Bar"),
        ];

        let mut test_table = create_table(&team_names);
        let home_team = team_names[0].as_str();
        let away_team = team_names[1].as_str();

        test_table.add_game(home_team, away_team, 2, 1);

        assert_eq!(test_table.teams[0].wins, 1);
        assert_eq!(test_table.teams[0].goal_scored, 2);
        assert_eq!(test_table.teams[0].goal_against, 1);

        assert_eq!(test_table.teams[1].defeats, 1);
        assert_eq!(test_table.teams[1].goal_scored, 1);
        assert_eq!(test_table.teams[1].goal_against, 2);
    }

    #[test]
    fn test_add_game_away_team_win() {
        let team_names = vec![
            String::from("FC Test"),
            String::from("Foo"),
            String::from("Bar"),
        ];

        let mut test_table = create_table(&team_names);
        let home_team = team_names[0].as_str();
        let away_team = team_names[1].as_str();

        test_table.add_game(home_team, away_team, 1, 2);

        assert_eq!(test_table.teams[0].defeats, 1);
        assert_eq!(test_table.teams[0].goal_scored, 1);
        assert_eq!(test_table.teams[0].goal_against, 2);

        assert_eq!(test_table.teams[1].wins, 1);
        assert_eq!(test_table.teams[1].goal_scored, 2);
        assert_eq!(test_table.teams[1].goal_against, 1);
    }

    #[test]
    fn test_add_game_draw() {
        let team_names = vec![
            String::from("FC Test"),
            String::from("Foo"),
            String::from("Bar"),
        ];

        let mut test_table = create_table(&team_names);
        let home_team = team_names[0].as_str();
        let away_team = team_names[1].as_str();

        test_table.add_game(home_team, away_team, 2, 2);

        assert_eq!(test_table.teams[0].draws, 1);
        assert_eq!(test_table.teams[0].goal_scored, 2);
        assert_eq!(test_table.teams[0].goal_against, 2);

        assert_eq!(test_table.teams[1].draws, 1);
        assert_eq!(test_table.teams[1].goal_scored, 2);
        assert_eq!(test_table.teams[1].goal_against, 2);
    }

    #[test]
    fn test_update_table() {
        let team_names = vec![
            String::from("FC Test"),
            String::from("Foo"),
            String::from("Bar"),
        ];

        let mut test_table = create_table(&team_names);
        let home_team = team_names[0].as_str();
        let away_team = team_names[1].as_str();

        test_table.add_game(home_team, away_team, 1, 2);
        test_table.update_table();
        assert_eq!(test_table.teams[0].points, 3);
        assert_eq!(test_table.teams[0].name, away_team);
        assert_eq!(test_table.teams[2].points, 0);
        assert_eq!(test_table.teams[2].name, home_team); // Team FC test should now be placed last since it has worse goal summart than Bar.
    }
}

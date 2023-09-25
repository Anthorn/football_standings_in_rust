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

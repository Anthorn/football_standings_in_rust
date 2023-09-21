extern crate chrono;

use chrono::Local;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

#[derive(Default, Debug)]
struct Team {
    name: String,
    played: u8,
    wins: u8,
    draws: u8,
    defeats: u8,
    goal_scored: i32,
    goal_against: i32,
    goal_difference: i32,
    points: u8,
}

struct Table {
    teams: Vec<Team>,
}

enum Commands {
    PrintTable,
    AddResult,
    ReadResultFile,
    ReadTableFile,
    SaveTableToFile,
    Exit,
}

impl Team {
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

impl Commands {
    fn from_string(s: &str) -> Option<Commands> {
        match s.to_lowercase().as_str() {
            "1" => Some(Commands::PrintTable),
            "2" => Some(Commands::AddResult),
            "3" => Some(Commands::ReadResultFile),
            "4" => Some(Commands::ReadTableFile),
            "5" => Some(Commands::SaveTableToFile),
            "exit" => Some(Commands::Exit),
            _ => None,
        }
    }
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
            let result: Option<&mut Team> =
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
            let result: Option<&mut Team> =
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

fn create_team_template() -> Team {
    Team {
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

fn create_table(teams: &[String]) -> Table {
    let team_template = create_team_template();
    let mut current_teams: Vec<Team> = vec![];
    for team in teams {
        let current_team = Team {
            name: team.clone(),
            ..team_template
        };
        current_teams.push(current_team);
    }

    Table {
        teams: current_teams,
    }
}

fn create_team(team_str: &String) -> Result<Team, &'static str> {
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
        Ok(Team {
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

fn read_table_from_file() -> Table {
    let mut file_name_input = String::new();
    println!("Input the table name: ");

    if std::io::stdin().read_line(&mut file_name_input).is_err() {
        println!("Cannot read input data.");
        return Table { teams: Vec::new() };
    }
    let mut current_dir = env::current_dir().unwrap();

    current_dir.push(&file_name_input.trim());

    match File::open(&current_dir) {
        Ok(read_file) => {
            let teams_and_errors: (
                Vec<Result<Team, &'static str>>,
                Vec<Result<Team, &'static str>>,
            ) = io::BufReader::new(read_file)
                .lines()
                .filter_map(|line| line.ok())
                .map(|line| create_team(&line))
                .partition(|created_teams| created_teams.is_ok());

            if teams_and_errors.1.len() > 0 {
                println!("Failed to parse {:?}", &current_dir);
                teams_and_errors
                    .1
                    .into_iter()
                    .for_each(|f| println!("{}", f.unwrap_err()));
                return Table { teams: Vec::new() };
            }

            let teams: Vec<Team> = teams_and_errors
                .0
                .into_iter()
                .map(|teams| teams.unwrap())
                .collect();

            println!("Table read successfully!");
            Table { teams: teams }
        }
        Err(_) => {
            println!("Couldn't parse table file with name {}", &file_name_input);
            Table { teams: Vec::new() }
        }
    }
}

fn parse_result(result_str: &String, table: &mut Table) {
    let parts: Vec<&str> = result_str.split(";").collect();

    if parts.len() != 2 {
        println!("Invalid result format.");
        return;
    }
    let teams: Vec<&str> = parts[0].split('-').collect();
    let score: Vec<&str> = parts[1].split('-').collect();

    if teams.len() != 2 || score.len() != 2 {
        println!("Invalid score format.");
        return;
    }

    let home_team = teams[0].trim();
    let away_team = teams[1].trim();

    let home_team_exists = table.teams.iter().any(|team| team.name == home_team);
    if home_team_exists == false {
        println!("Home team does not exist.");
        return;
    }

    let away_team_exists = table.teams.iter().any(|team| team.name == away_team);
    if away_team_exists == false {
        println!("Away team does not exist.");
        return;
    }

    let goal_home_team_result = score[0].trim().parse::<i32>();
    let goal_away_team_result = score[1].trim().parse::<i32>();

    let goal_home_team: i32 = match goal_home_team_result {
        Ok(value) => value,
        Err(_) => {
            println!("Cannot parse the home team score.");
            return;
        }
    };
    let goal_away_team: i32 = match goal_away_team_result {
        Ok(value) => value,
        Err(_) => {
            println!("Cannot parse the away team score.");
            return;
        }
    };

    println!(
        "Adding {} - {} {}-{}",
        home_team, away_team, goal_home_team, goal_away_team
    );

    table.add_game(home_team, away_team, goal_home_team, goal_away_team);
}

fn add_result(table: &mut Table) {
    println!("Add result(team1-team2;xx-xx). Input \"done\" when you are finished.");

    let mut result_input = String::new();
    while result_input != "done" {
        result_input.clear();
        if let Ok(_) = io::stdin().read_line(&mut result_input) {
            result_input.pop();

            if result_input == "done" {
                return;
            }
            parse_result(&result_input, table);
        }
    }
}

fn read_result_from_file(table: &mut Table) -> std::io::Result<()> {
    println!("Reading result file...");

    let read_file = File::open("results.txt")?;

    let reader: BufReader<File> = BufReader::new(read_file);

    for line in reader.lines() {
        let line_contents = line?;
        parse_result(&line_contents, table);
    }

    Ok(())
}

fn save_table_to_file(table: &Table) -> std::io::Result<()> {
    let frm_date_time_now = Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    println!("table_{}", frm_date_time_now);

    let filename = format!("table_{}.txt", frm_date_time_now);

    let mut file = File::create(filename)?;

    for team in &table.teams {
        let current_team_str = format!(
            "{};{};{};{};{};{};{}\n",
            team.name.clone(),
            team.played,
            team.wins,
            team.draws,
            team.defeats,
            team.goal_scored,
            team.goal_against
        );
        let result = file.write_all(current_team_str.as_bytes());
        match result {
            Ok(_) => println!("{current_team_str} parsed ok."),
            Err(_) => println!("{current_team_str} Failed to parse."),
        }
    }

    Ok(())
}

fn main() {
    //TODO: Read these teams from file instead.
    let teams: [String; 16] = [
        String::from("AIK"),
        String::from("BP"),
        String::from("Djurgården"),
        String::from("Degerfors"),
        String::from("Elfsborg"),
        String::from("Häcken"),
        String::from("Hammarby"),
        String::from("Halmstad BK"),
        String::from("IFK Göteborg"),
        String::from("Kalmar FF"),
        String::from("Malmö FF"),
        String::from("Mjällby"),
        String::from("Norrköping"),
        String::from("Varbergs BOIS"),
        String::from("Värnamo"),
        String::from("Sirius"),
    ];
    let mut table: Table = create_table(&teams);

    let mut inputs = String::new();

    loop {
        println!("Choose your option!");
        println!("1. Print table.");
        println!("2. Add new result.");
        println!("3. Read multiple results from file.");
        println!("4. Read table from file.");
        println!("5. Save current table.");

        inputs.clear();
        table.update_table();

        if let Ok(_) = io::stdin().read_line(&mut inputs) {
            inputs.pop();

            let command = Commands::from_string(&inputs);

            match command {
                Some(Commands::PrintTable) => table.print(),
                Some(Commands::AddResult) => add_result(&mut table),
                Some(Commands::ReadResultFile) => read_result_from_file(&mut table).unwrap(),
                Some(Commands::ReadTableFile) => table = read_table_from_file(),
                Some(Commands::SaveTableToFile) => save_table_to_file(&table).unwrap(),
                Some(Commands::Exit) => return,
                None => println!("Unknown command."),
            }
        }
    }
}

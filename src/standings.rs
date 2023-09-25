use chrono::Local;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

pub mod tables;
pub mod team;

fn parse_result(result_str: &String, table: &mut tables::Table) {
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

pub fn add_result(table: &mut tables::Table) {
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

pub fn read_result_from_file(table: &mut tables::Table) -> std::io::Result<()> {
    println!("Reading result file...");

    let read_file = File::open("results.txt")?;

    let reader: BufReader<File> = BufReader::new(read_file);

    for line in reader.lines() {
        let line_contents = line?;
        parse_result(&line_contents, table);
    }

    Ok(())
}

pub fn save_table_to_file(table: &tables::Table) -> std::io::Result<()> {
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

pub fn read_table_from_file() -> tables::Table {
    let mut file_name_input = String::new();
    println!("Input the table name: ");

    if std::io::stdin().read_line(&mut file_name_input).is_err() {
        println!("Cannot read input data.");
        return tables::Table { teams: Vec::new() };
    }
    let mut current_dir = env::current_dir().unwrap();

    current_dir.push(&file_name_input.trim());

    match File::open(&current_dir) {
        Ok(read_file) => {
            let teams_and_errors: (
                Vec<Result<team::TeamStats, &'static str>>,
                Vec<Result<team::TeamStats, &'static str>>,
            ) = io::BufReader::new(read_file)
                .lines()
                .filter_map(|line| line.ok())
                .map(|line| team::create_team(&line))
                .partition(|created_teams| created_teams.is_ok());

            if teams_and_errors.1.len() > 0 {
                println!("Failed to parse {:?}", &current_dir);
                teams_and_errors
                    .1
                    .into_iter()
                    .for_each(|f| println!("{}", f.unwrap_err()));
                return tables::Table { teams: Vec::new() };
            }

            let teams: Vec<team::TeamStats> = teams_and_errors
                .0
                .into_iter()
                .map(|teams| teams.unwrap())
                .collect();

            println!("Table read successfully!");
            tables::Table { teams: teams }
        }
        Err(_) => {
            println!("Couldn't parse table file with name {}", &file_name_input);
            tables::Table { teams: Vec::new() }
        }
    }
}

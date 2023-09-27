extern crate chrono;
use std::io;

// Import crates/functions.
pub mod standings;

enum Commands {
    PrintTable,
    AddResult,
    ReadResultFile,
    ReadTableFile,
    SaveTableToFile,
    Exit,
}

impl Commands {
    fn from_string(s: &str) -> Option<Commands> {
        match s.to_lowercase().as_str() {
            "1" => Some(Commands::PrintTable),
            "2" => Some(Commands::AddResult),
            "3" => Some(Commands::ReadResultFile),
            "4" => Some(Commands::ReadTableFile),
            "5" => Some(Commands::SaveTableToFile),
            "6" => Some(Commands::Exit),
            _ => None,
        }
    }
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
    let mut table: standings::tables::Table = standings::tables::create_table(&teams);

    let mut inputs = String::new();

    loop {
        println!("Choose your option!");
        println!("1. Print table.");
        println!("2. Add new result.");
        println!("3. Read multiple results from file.");
        println!("4. Read table from file.");
        println!("5. Save current table.");
        println!("6. Exit.");

        inputs.clear();
        table.update_table();

        if let Ok(_) = io::stdin().read_line(&mut inputs) {
            inputs.pop();

            let command = Commands::from_string(&inputs);

            match command {
                Some(Commands::PrintTable) => table.print(),
                Some(Commands::AddResult) => standings::add_result(&mut table),
                Some(Commands::ReadResultFile) => {
                    standings::read_result_from_file(&mut table).unwrap()
                }
                Some(Commands::ReadTableFile) => table = standings::read_table_from_file(),
                Some(Commands::SaveTableToFile) => standings::save_table_to_file(&table).unwrap(),
                Some(Commands::Exit) => return,
                None => println!("Unknown command."),
            }
        }
    }
}

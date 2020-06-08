use crate::utils::table::Table;
use crate::utils::player::Player;

#[derive(Debug)]
pub struct Round {
    pub table: Table,
    pub players: Option<Vec<Player>>,
}

impl Round {
    pub fn new() -> Round {
        let table: Table = Round::new_table();

        Round {
            table: table,
            players: None,
        }
    }

    pub fn new_table() -> Table {
        Table::new()
    }

    pub fn clean_table(&mut self) {
        self.table.clean();
    }

    pub fn add_players(&mut self, players: Vec<Player>) {
        self.players = Some(players);
    }

    pub fn start_round(&mut self, round_number: &mut u32) {
        if let Some(_players) = &mut self.players {
            println!("Starting new round.");
        } else {
            println!("There is no players in this round.");
        }
    }
}
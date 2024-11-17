use diesel_demo::read_players_file;

fn main() {
    read_players_file().expect("Failed to read file");
}

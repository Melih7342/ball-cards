#[derive(Debug, PartialEq)]
enum DanceStyle {
    Walzer,
    Polka,
    Menuett,
    GermanDance,
}

#[derive(Debug)]
struct DanceSlot {
    id: u8,
    style: DanceStyle,
    partner: Option<String>,
}

#[derive(Debug)]
struct DanceCard {
    owner: String,
    slots: Vec<DanceSlot>,
}

fn main() {

}
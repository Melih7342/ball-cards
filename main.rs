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

impl DanceCard {
    fn new(owner_name: String) -> Self {
        let initial_slots = vec![
                DanceSlot { id: 1, style: DanceStyle::Walzer, partner: None },
                DanceSlot { id: 2, style: DanceStyle::GermanDance, partner: None },
                DanceSlot { id: 3, style: DanceStyle::Polka, partner: None },
            ];
        
        Self {
            owner: owner_name,
            slots: initial_slots,
        }
    }
}

fn main() {

}
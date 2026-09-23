use std::fmt::Error;

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

    fn book_dance(&mut self, slot_id: u8, partner_name: String) -> Result<(), String> {
        for slot in &mut self.slots {
            if slot.id == slot_id {
                match slot.partner {
                    Some(_) => return Err("The dance is taken.".to_string()),
                    None => {
                        slot.partner = Some(partner_name);
                        return Ok(())
                    } 
                }
            }
        }
        Err("The dance isn't on the dance card!".to_string())
    }
}

fn main() {

}
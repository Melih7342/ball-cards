use std::fmt::Error;
use std::sync::{Arc, Mutex};
use std::thread;

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
    let mut lottes_card = DanceCard::new(String::from("Lotte"));

    let shared_card = Arc::new(Mutex::new(lottes_card));

    let card_for_werther = Arc::clone(&shared_card);

    let thread_werther = thread::spawn(move || {
        let mut card = card_for_werther.lock().unwrap();

        let result = card.book_dance(1, String::from("Werther"));
        println!("Werther tries to book Slot 1: {:?}", result);    
    });

    let card_for_albert = Arc::clone(&shared_card);
    
    let thread_albert = thread::spawn(move || {
        let mut card = card_for_albert.lock().unwrap();
        
        let result = card.book_dance(1, String::from("Albert"));
        println!("Albert versucht Slot 1 zu buchen: {:?}", result);
    });

    thread_werther.join().unwrap();
    thread_albert.join().unwrap();

    println!("{:#?}", shared_card.lock().unwrap());
}
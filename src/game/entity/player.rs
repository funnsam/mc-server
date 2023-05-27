use super::*;
use super::super::item::*;


#[derive(Clone, Debug)]
pub struct PlayerBehavior {
    pub inventory: Vec<Item>,
    pub selected_hb: i16
}

unsafe impl Send for PlayerBehavior {}
unsafe impl Sync for PlayerBehavior {}

impl super::EntityBehavior for PlayerBehavior {
    fn new(_: i32) -> Self {
        Self {
            inventory: vec![Item::default(); 46],
            selected_hb: 0
        }
    }
    
    // 0: set inventory item ( slot: i16, id: i16, qty: i16, meta: i16 )
    // 1: get inventory item ( slot: i16 ) -> ( id: i16, meta: i16, qty: i16 )
    // 2: set selected slot ( slot: i16 )
    // 3: get selected slot ( ) -> ( slot: i16 )
    fn do_action(&mut self, k: u32, args: &[u8], e: &mut Entity) -> Option<Vec<u8>> {
        match k {
            0 => {
                let sl = i16::from_be_bytes(args[0..=1].try_into().unwrap());
                let id = i16::from_be_bytes(args[2..=3].try_into().unwrap());
                let qt = i16::from_be_bytes(args[4..=5].try_into().unwrap());
                let dm = i16::from_be_bytes(args[6..=7].try_into().unwrap());
                self.inventory[sl as usize] = Item {
                    id,
                    meta: dm,
                    qty: qt
                };
                None
            },
            1 => {
                let sl = i16::from_be_bytes(args[0..=1].try_into().unwrap());
                let i = &self.inventory[sl as usize];
                Some([
                    i.id.to_be_bytes(),
                    i.meta.to_be_bytes(),
                    i.qty.to_be_bytes(),
                ].concat())
            },
            2 => {
                self.selected_hb = i16::from_be_bytes(args[0..=1].try_into().unwrap());
                None
            },
            3 => Some(self.selected_hb.to_be_bytes().to_vec()),
            _ => None
        }
    }
}

use super::*;

#[derive(Clone, Debug)]
pub struct PlayerBehavior {
    // pub inventory: Vec<Item>
}

unsafe impl Send for PlayerBehavior {}
unsafe impl Sync for PlayerBehavior {}

impl super::EntityBehavior for PlayerBehavior {
    fn new(_: &mut Entity, _: i32) -> Self {
        Self {}
    }
}

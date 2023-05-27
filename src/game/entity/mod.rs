use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Entity {
    pub position: EntityPosition,
    pub rotation: EntityRotation,
    // pub health: i32,
    pub behavior: Arc<Box<dyn EntityBehavior>>
}

impl Entity {
    pub fn new(eb: Box<dyn EntityBehavior>) -> Self {
        Self {
            position: EntityPosition { x: 0.0, y: 0.0, z: 0.0 },
            rotation: EntityRotation { y: 0.0, p: 0.0 },
            behavior: Arc::new(eb)
        }
    }
}

#[derive(Clone, Debug)]
pub struct EntityPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Clone, Debug)]
pub struct EntityRotation {
    pub y: f32,
    pub p: f32
}

// use crate::protocol::packet::*;

pub trait EntityBehavior: std::fmt::Debug + Send + Sync {
    fn new(e: &mut Entity, id: i32) -> Self where Self: Sized;
}

pub mod player;

use super::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Behaviour {
    fn constructor(&mut self, entity: &mut Ent) -> ();
    fn process(&mut self, entity: &mut Ent) -> ();
    fn deconstructor(&mut self, entity: &mut Ent) -> ();
}

#[derive(Debug)]
pub struct Entity {
    soul: Rc<RefCell<Ent>>
}

pub struct Ent {
    pub position: (i32, i32),
    pub sprite: Sprite,
    ///RefCells to ensure interior mutibility. Only None if parent is this entity is the child of the app.
    parent: Option<Entity>,
    children: Vec<Entity>,
    behaviour: Box<dyn Behaviour>,
}

impl Ent {
    pub fn new(behaviour: Box<dyn Behaviour>, position: (i32, i32), sprite: Sprite) -> Ent {
        Ent {
            position,
            sprite,
            parent: None,
            children: vec![],
            behaviour: behaviour,
        }
    }
}

impl std::fmt::Debug for Ent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Ent")
            .field("position", &self.position)
            .field("sprite", &self.sprite)
            .field("parent", &self.parent)
            .field("children", &self.children)
            .finish()
    }
}

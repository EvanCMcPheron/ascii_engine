use getset::{Getters, MutGetters};
use super::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Behaviour {
    fn constructor(&mut self, entity: &mut Soul) -> ();
    fn process(&mut self, entity: &mut Soul) -> ();
    fn deconstructor(&mut self, entity: &mut Soul) -> ();
}

#[derive(Debug, Getters)]
pub struct Entity {
    #[getset(get)]
    soul: Rc<RefCell<Soul>>
}

impl Entity {
    pub fn new(behaviour: Box<dyn Behaviour>, position: (i32, i32), sprite: Sprite) -> Entity {
        Entity { soul: Rc::new(
            RefCell::new(Soul::new(behaviour, position, sprite))
        ) }
    }
}


#[derive(Getters, MutGetters)]
pub struct Soul {
    #[getset(get, get_mut)]
    position: (i32, i32),
    #[getset(get, get_mut)]
    sprite: Sprite,
    ///RefCells to ensure interior mutibility. Only None if parent is this entity is the child of the app.
    #[getset(get, get_mut)]
    parent: Option<Entity>,
    #[getset(get, get_mut)]
    children: Vec<Entity>,
    #[getset(get, get_mut)]
    behaviour: Box<dyn Behaviour>,
}

impl Soul {
    pub fn new(behaviour: Box<dyn Behaviour>, position: (i32, i32), sprite: Sprite) -> Soul {
        Soul {
            position,
            sprite,
            parent: None,
            children: vec![],
            behaviour: behaviour,
        }
    }
    
}

impl std::fmt::Debug for Soul {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Soul")
            .field("position", &self.position)
            .field("sprite", &self.sprite)
            .field("parent", &self.parent)
            .field("children", &self.children)
            .finish()
    }
}

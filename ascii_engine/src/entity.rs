use super::prelude::*;
use anyhow::{bail, ensure};
use getset::{Getters, MutGetters};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Behaviour {
    fn constructor(&mut self, entity: &mut Entity) -> Result<()>;
    fn process(&mut self, entity: &mut Entity) -> Result<()>;
    fn deconstructor(&mut self, entity: &mut Entity) -> Result<()>;
}

#[derive(Debug)]
pub struct Entity {
    soul: Rc<RefCell<Soul>>,
}

impl Entity {
    pub fn new(behaviour: Box<dyn Behaviour>, position: (i32, i32), sprite: Sprite) -> Entity {
        Entity {
            soul: Rc::new(RefCell::new(Soul::new(behaviour, position, sprite))),
        }
    }
    pub fn soul(&self) -> Result<std::cell::Ref<Soul>> {
        match self.soul.try_borrow() {
            Ok(r) => Ok(r),
            Err(_) => bail!("Could not borrow soul, it is already mutibly borrowed."),
        }
    }
    pub fn soul_mut(&mut self) -> Result<std::cell::RefMut<Soul>> {
        match self.soul.try_borrow_mut() {
            Ok(r) => Ok(r),
            Err(_) => bail!("Could not borrow soul, it is already mutibly borrowed."),
        }
    }
    pub fn twin(&self) -> Entity {
        Entity {
            soul: Rc::clone(&self.soul),
        }
    }
    pub fn orphan(&mut self) -> Result<()> {
        match self.soul.try_borrow_mut() {
            Ok(mut soul) => {
                match &mut (*soul).parent {
                    Some(parent) => match parent.soul.try_borrow_mut() {
                        Ok(mut parent_soul) => {
                            let indx = (*parent_soul).children().iter().enumerate().fold(
                                None,
                                |accum, (i, ent)| {
                                    if ent.soul.as_ptr() == self.soul.as_ptr() {
                                        Some(i)
                                    } else {
                                        accum
                                    }
                                },
                            );
                            println!("Childs indx in parent: {:?}", indx);
                            if let Some(i) = indx {
                                (*parent_soul).children_mut().remove(i);
                            }
                        }
                        Err(_) => {
                            bail!("Could not mutibly borrow parent's soul, it is already borrowed")
                        }
                    },
                    None => (),
                }
                (*soul).parent = None;
                Ok(())
            }
            Err(_) => bail!("Could not mutibly borrow soul, it is already borrowed"),
        }
    }
    pub fn add_child(&mut self, child: &mut Entity) -> Result<()> {
        child.orphan()?; //Garentees that souls will only exist at a single location in the tree
        let new_ent = child.twin();
        (*new_ent.soul.borrow_mut()).parent = Some(self.twin());
        match self.soul.try_borrow_mut() {
            Ok(mut soul) => {
                (*soul).children.push(new_ent);
                Ok(())
            }
            Err(_) => bail!("Could not mutibly borrow soul, it is already borrowed"),
        }
    }
    pub fn get_child_indx(&self, child: &Entity) -> Option<usize> {
        match self.soul() {
            Ok(soul) => soul
                .children()
                .iter()
                .enumerate()
                .fold(None, |accum, (i, ent)| {
                    if ent.soul_ref().as_ptr() == child.soul_ref().as_ptr() {
                        Some(i)
                    } else {
                        accum
                    }
                }),
            Err(_) => None,
        }
    }
    pub fn soul_ref(&self) -> &Rc<RefCell<Soul>> {
        &self.soul
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
            .field("parent", &"Hidden Value")
            .field("children", &self.children)
            .finish()
    }
}
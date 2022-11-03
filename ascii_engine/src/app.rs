use super::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use anyhow::{bail, ensure};

#[derive(Debug)]
pub struct App {
    pub dimensions: (u32, u32),
    children: Vec<Entity>,
}
impl App {
    pub fn new(dimensions: (u32, u32)) -> App{
        App{
            dimensions,
            children: vec![],
        }
    }
    pub fn add_child(&mut self, child: &Entity) -> Result<()> {
        self.children.push(child.twin());
        Ok(())
    }
    pub fn remove_child(&mut self, child: &Entity) -> Result<()> {
        if let Some(i) = self.get_child_indx(child) {
            self.children.remove(i);
            return Ok(());
        }
        bail!("Could not find a matching child.");
    }
    pub fn get_child_indx(&self, child: &Entity) -> Option<usize> {
        self.children.iter().enumerate().fold(None, |accum, (i, x)| if child == x {Some(i)} else {accum})
    }
    pub fn run(&mut self) -> Result<()> {
        for child in self.children.iter_mut() {
            let mut twin = child.twin();
            child.soul_mut()?.behaviour_mut().c(&mut twin)?;
        }
        Ok(())
    }
}
use super::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct App {
    pub dimensions: (u32, u32),
    child: RefCell<Vec<Rc<Ent>>>,
}

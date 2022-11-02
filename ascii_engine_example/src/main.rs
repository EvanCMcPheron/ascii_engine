use ascii_engine::prelude::*;

struct ExampleType();
impl Behaviour for ExampleType {
    fn constructor(&mut self, _entity: &mut Soul) -> () {
      ()
    }
    fn process(&mut self, _entity: &mut Soul) -> () {
      ()
    }
    fn deconstructor(&mut self, _entity: &mut Soul) -> () {
      ()
    }
}

fn main() {
    let brain = ExampleType();
    let entity = Entity::new(Box::new(brain), (0, 0), Sprite::default());
    println!("{:?}", entity);
}

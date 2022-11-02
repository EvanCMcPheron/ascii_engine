use ascii_engine::prelude::*;

struct ExampleType();
impl Behaviour for ExampleType {
    fn constructor(&mut self, _entity: &mut Ent) -> () {
      ()
    }
    fn process(&mut self, _entity: &mut Ent) -> () {
      ()
    }
    fn deconstructor(&mut self, _entity: &mut Ent) -> () {
      ()
    }
}

fn main() {
    let brain = ExampleType();
    let entity = Ent::new(Box::new(brain), (0, 0), Sprite::default());
    println!("{:?}", entity);
}

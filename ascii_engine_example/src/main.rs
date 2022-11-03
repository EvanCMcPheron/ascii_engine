use ascii_engine::prelude::*;

struct ExampleType();
impl Behaviour for ExampleType {
    fn constructor(&mut self, _entity: &mut Entity) -> () {
      ()
    }
    fn process(&mut self, _entity: &mut Entity) -> () {
      ()
    }
    fn deconstructor(&mut self, _entity: &mut Entity) -> () {
      ()
    }
}

fn main() ->  Result<()>{
    let brain1 = ExampleType();
    let entity1 = Entity::new(Box::new(brain1), (0, 0), Sprite::default());
    let brain2 = ExampleType();
    let entity2 = Entity::new(Box::new(brain2), (0, 0), Sprite::default());
    entity1.add_child(&entity2)?;
    println!("{:#?}", entity1);
    entity2.orphan()?;
    println!("{:#?}", entity1);
    println!("{:#?}", entity2);
    Ok(())
}

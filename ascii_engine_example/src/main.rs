use ascii_engine::prelude::*;

struct ExampleType();
impl Behaviour for ExampleType {
    fn constructor(&mut self, _entity: &mut Entity) -> () {
        println!("brain1");
    }
    fn process(&mut self, _entity: &mut Entity) -> () {
    }
    fn deconstructor(&mut self, _entity: &mut Entity) -> () {
    }
}

fn main(){
    let brain1 = ExampleType();
    let brain2 = Brain::new(|x| {println!("brain2");}, |x| (), |x| ());
    let mut ent1 = Entity::new(Box::new(brain1), (0,0), Sprite::default());
    let mut ent2 = Entity::new(Box::new(brain2), (0,0), Sprite::default());
    ent1.add_child(&mut ent2).unwrap();

    let mut app = App::new((5, 5));
    app.add_child(&mut ent1);

    //println!("{:#?}", app);
    ent1.remove_child(&mut ent2);
    //println!("{:#?}", app);
    app.remove_child(&mut ent1);
    //println!("{:#?}", app);

    app.run();
}

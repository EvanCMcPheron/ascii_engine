use ascii_engine::prelude::*;

struct ExampleType();
impl Behaviour for ExampleType {
    fn constructor(&mut self, _entity: &mut Entity) -> () {
    }
    fn process(&mut self, _entity: &mut Entity) -> () {
    }
    fn deconstructor(&mut self, _entity: &mut Entity) -> () {
    }
}

fn main(){
    let brain1 = ExampleType();
    let brain2 = Brain::new(|x| (), |x| (), |x| ());
}

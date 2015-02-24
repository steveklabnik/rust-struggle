pub mod components;
use components::{BasicComponent, Component};

fn main () {
  let cmp: BasicComponent = components::Component::new(10);
  let id = cmp.id;

  println!("component id: {}", id);
  println!("done");

}

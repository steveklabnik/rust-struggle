pub mod components;

fn main () {

  let mut cmp = components::Component::new(10);
  let id = cmp.id();

  println!("component id: {}", id);
  println!("done");

}

mod components;

fn main () {

  let mut cmp = components::BasicComponent::new(10);

  println!("component id: {}", cmp.id);
  println!("done");

}

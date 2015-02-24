pub trait Component {

  fn new (u32) -> Self;

  fn id (&self) -> u32;

}

pub struct BasicComponent {

  id: u32

}

impl Component for BasicComponent {

  fn new (id : u32) -> Self {
    BasicComponent {
      id: id
    }
  }

  fn id (&self) -> u32 {
    self.id
  }

}

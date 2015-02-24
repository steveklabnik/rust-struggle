trait Component {

  fn new (u32) -> Self;

}

struct BasicComponent {

  id: u32

}

impl Component for BasicComponent {

  fn new (id : u32) -> Self {
    BasicComponent {
      id: id
    }
  }

}

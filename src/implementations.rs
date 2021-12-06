pub fn run() {
  struct Box {
    height: i32,
    width: i32,
    depth: i32
  }

  impl Box {
    fn get_volume(&self) -> i32 {
      return self.height * self.width * self.depth;
    }

    fn small() -> Self {
      return Self {
        height: 2,
        width: 2,
        depth: 1
      }
    }

    fn large() -> Self {
      return Self {
        height: 4,
        width: 4,
        depth: 2
      }
    }
  }

  let my_small_box_volume = Box::small().get_volume();
  let my_large_box_volume = Box::large().get_volume();

  println!("Small capacity: {:?}", my_small_box_volume);
  println!("Large capacity: {:?}", my_large_box_volume);
}
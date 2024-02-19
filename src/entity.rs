pub struct Entity32 {
  pub id: u32,
}

impl Entity32 {
  pub fn new(id: u32, position: (f32, f32, f32)) -> Self {
      Entity { id, position }
  }
}

pub struct Entity64 {
  pub id: u64,
}

impl Entity64 {
  pub fn new(id: u32, position: (f32, f32, f32)) -> Self {
      Entity { id, position }
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Entity32 {
  pub id: u32,
}

impl Entity32 {
  pub fn new(id: u32) -> Self {
    Entity32 { id }
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Entity64 {
  pub id: u64,
}

impl Entity64 {
  pub fn new(id: u64) -> Self {
    Entity64 { id }
  }
}

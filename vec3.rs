#[derive(Copy, Clone)]
pub struct Vec3 {
  pub e0: f32,
  pub e1: f32,
  pub e2: f32,
}

impl Vec3 {
  pub fn add(self, v: Vec3) -> Vec3 {
    return Vec3 {
      e0: self.e0 + v.e0,
      e1: self.e1 + v.e1,
      e2: self.e2 + v.e2,
    };
  }

  pub fn sub(self, v: Vec3) -> Vec3 {
    return Vec3 {
      e0: self.e0 - v.e1,
      e1: self.e1 - v.e1,
      e2: self.e2 - v.e1,
    };
  }

  pub fn scalar_mul(self, t: f32) -> Vec3 {
    return Vec3 {
      e0: self.e0 * t,
      e1: self.e1 * t,
      e2: self.e2 * t,
    };
  }

  pub fn scalar_div(self, t: f32) -> Vec3 {
    return self.scalar_mul(1.0 / t);
  }

  pub fn dot(self, v: Vec3) -> f32 {
    return self.e0 * v.e0 + self.e1 * v.e1 + self.e2 * v.e2;
  }

  pub fn cross(self, v: Vec3) -> Vec3 {
    return Vec3 {
      e0: self.e1 * v.e2 - self.e2 * v.e1,
      e1: -(self.e0 * v.e2 - self.e2 * v.e0),
      e2: self.e0 * v.e1 - self.e1 * v.e0
    };
  }

  pub fn length(self) -> f32 {
    return self.dot(self).sqrt();
  }
}
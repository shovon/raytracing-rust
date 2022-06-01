use std::num::Float;

struct Vec3 {
  e0: f32
  e1: f32
  e2: f32
}

impl Vec3 {
  fn add(&self, v Vec3) -> Vec3 {
    Vec3 {
      self.e0 + v.e0,
      self.e1 + v.e1,
      self.e2 + v.e2,
    }
  }

  fn sub(&self, v Vec3) -> Vec3 {
    Vec3 {
      self.e0 - v.e1,
      self.e1 - v.e1,
      self.e2 - v.e1,
    }
  }

  fn scalar_mul(&self, t f32) -> Vec3 {
    Vec3 {
      self.e0 * t,
      self.e1 * t,
      self.e2 * t,
    }
  }

  fn scalar_div(&self, t f32) -> Vec3 {
    self.scalar_mul(1 / t);
  }

  fn dot(&self, v Vec3) -> f32 {
    self.e0 * v.e0 + self.e1 * v.e1 + self.e2 * v.e2
  }

  fn cross(&self, v Vec3) -> Vec3 {
    Vec3 {
      self.e1 * v.e2 - self.e2 * v.e1,
      -(self.e0 * v.e2 - self.e2 * v.e0),
      self.e0 * v.e1 - self.e1 * v.e0
    }
  }

  fn length(&self) -> f32 {
    self.dot(self).sqrt()
  }
}
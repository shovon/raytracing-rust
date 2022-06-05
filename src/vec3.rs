#[derive(Copy, Clone)]
pub struct Vec3 {
  pub e0: f32,
  pub e1: f32,
  pub e2: f32,
}

fn rand_range(min: f32, max: f32) -> f32 {
  rand::random::<f32>() * (max - min) + min
}

impl Vec3 {
  pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 {
      e0: x,
      e1: y,
      e2: z,
    }
  }

  pub fn zero() -> Vec3 {
    Vec3 {
      e0: 0.0,
      e1: 0.0,
      e2: 0.0,
    }
  }

  pub fn add(self, v: Vec3) -> Vec3 {
    Vec3 {
      e0: self.e0 + v.e0,
      e1: self.e1 + v.e1,
      e2: self.e2 + v.e2,
    }
  }

  pub fn sub(self, v: Vec3) -> Vec3 {
    Vec3 {
      e0: self.e0 - v.e0,
      e1: self.e1 - v.e1,
      e2: self.e2 - v.e2,
    }
  }

  pub fn random_min_max(min: f32, max: f32) -> Vec3 {
    Vec3 {
      e0: rand_range(min, max),
      e1: rand_range(min, max),
      e2: rand_range(min, max),
    }
  }

  pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::random_min_max(-1.0, 1.0);
    while p.dot(p) >= 1.0 {
      p = Vec3::random_min_max(-1.0, 1.0);
    }
    p
  }

  pub fn scalar_mul(self, t: f32) -> Vec3 {
    Vec3 {
      e0: self.e0 * t,
      e1: self.e1 * t,
      e2: self.e2 * t,
    }
  }

  pub fn scalar_div(self, t: f32) -> Vec3 {
    self.scalar_mul(1.0 / t)
  }

  pub fn dot(self, v: Vec3) -> f32 {
    self.e0 * v.e0 + self.e1 * v.e1 + self.e2 * v.e2
  }

  pub fn cross(self, v: Vec3) -> Vec3 {
    Vec3 {
      e0: self.e1 * v.e2 - self.e2 * v.e1,
      e1: -(self.e0 * v.e2 - self.e2 * v.e0),
      e2: self.e0 * v.e1 - self.e1 * v.e0,
    }
  }

  pub fn length(self) -> f32 {
    self.dot(self).sqrt()
  }

  pub fn unit_vector(self) -> Vec3 {
    self.scalar_div(self.length())
  }
}

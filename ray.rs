use vec3::Vec3;

pub struct Ray {
  pub a: Vec3,
  pub b: Vec3,
}

impl Ray {
  pub fn origin(self) -> Vec3 {
    return self.a;
  }

  pub fn direction(self) -> Vec3 {
    return self.b;
  }

  pub fn point_at_parameter(self, t: f32) -> Vec3 {
    return self.a.add(self.b.scalar_mul(t));
  }
}

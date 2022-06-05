use crate::vec3::Vec3;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v.sub(n.scalar_mul(2.0 * v.dot(n)))
}

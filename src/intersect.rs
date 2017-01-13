use math::*;

pub trait Intersect {
    fn intersect(&self, r: &Ray3) -> Option<Vec3>;
}
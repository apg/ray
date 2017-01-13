use math::*;
use intersect::*;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub r: f64,
}

impl Sphere {
    pub fn from_radius(r: f64) -> Sphere {
        Sphere::new(r, Vec3::origin())
    }

    pub fn new(r: f64, center: Vec3) -> Sphere {
        Sphere { r: r, center: center }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, r: &Ray3) -> Option<Vec3> {
        let w = r.o.sub(&self.center);
        let a = r.dir.dot(&r.dir);
        let b = w.scale(2.0).dot(&r.dir);
        let c = w.dot(&w) - (self.r * self.r);
        let d = (b * b) - (4.0 * a * c);

        if d >= 0.0 {
            let t = (-b - d.sqrt()) / (2.0_f64 * a);
            let rs = r.dir.scale(t);
            Some(r.o.add(&rs))
        } else {
            None
        }
    }
}

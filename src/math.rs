#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn origin() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn dot(&self, o: &Vec3) -> f64 {
        (self.x * o.x) + (self.y * o.y) + (self.z * o.z)
    }

    pub fn add(&self, o: &Vec3) -> Vec3 {
        Vec3::new(self.x + o.x , self.y + o.y, self.z + o.z)
    }

    pub fn sub(&self, o: &Vec3) -> Vec3 {
        Vec3::new(self.x - o.x , self.y - o.y, self.z - o.z)
    }
    
    pub fn scale(&self, f: f64) -> Vec3 {
        Vec3::new(self.x * f , self.y * f, self.z * f)
    }

    fn mag(&self) -> f64 {
        let t = (self.x * self.x) +
            (self.y * self.y) +
            (self.z * self.z);
        t.sqrt()
    }
}

#[derive(Debug)]
pub struct Ray3 {
    pub o: Vec3,
    pub dir: Vec3,
}

impl Ray3 {
    pub fn from_origin(dir: Vec3) -> Ray3 {
        Ray3::new(Vec3::origin(), dir)
    }

    pub fn new(o: Vec3, dir: Vec3) -> Ray3 {
        Ray3 { o: o, dir: dir }
    }
}


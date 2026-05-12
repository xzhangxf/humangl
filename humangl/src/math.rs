#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn sub(a: Self, b: Self) -> Self {
        Self { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z }
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Self, b: Self) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn length(self) -> f32 {
        Vec3::dot(self, self).sqrt()
    }

    pub fn normalise(self) -> Self {
        let l = self.length();
        if l <= f32::EPSILON {
            return Self::new(0.0, 1.0, 0.0);
        }
        Self {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub m: [f32; 16],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self {
            m: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.m.as_ptr()
    }

    #[inline]
    fn at(&self, row: usize, col: usize) -> f32 {
        self.m[row + col * 4]
    }

    #[inline]
    fn set(&mut self, row: usize, col: usize, v: f32) {
        self.m[row + col * 4] = v;
    }

    pub fn mul(a: Mat4, b: Mat4) -> Mat4 {
        let mut c = Mat4 { m: [0.0; 16] };

        for col in 0..4 {
            for row in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += a.at(row, k) * b.at(k, col);
                }
                c.set(row, col, sum);
            }
        }
        c
    }

    pub fn translation(tx: f32, ty: f32, tz: f32) -> Mat4 {
        let mut t = Mat4::identity();
        t.set(0, 3, tx);
        t.set(1, 3, ty);
        t.set(2, 3, tz);
        t
    }

    pub fn rotation_y(radians: f32) -> Mat4 {
        let c = radians.cos();
        let s = radians.sin();

        let mut r = Mat4::identity();
        r.set(0, 0, c);
        r.set(2, 0, -s);
        r.set(0, 2, s);
        r.set(2, 2, c);
        r
    }

    pub fn scale(sx: f32, sy: f32, sz: f32) -> Mat4 {
        let mut s = Mat4::identity();
        s.set(0, 0, sx);
        s.set(1, 1, sy);
        s.set(2, 2, sz);
        s
    }

    pub fn rotation_x(radians: f32) -> Mat4 {
        let c = radians.cos();
        let s = radians.sin();

        let mut r = Mat4::identity();
        r.set(1, 1, c);
        r.set(2, 1, s);
        r.set(1, 2, -s);
        r.set(2, 2, c);
        r
    }

    pub fn rotation_z(radians: f32) -> Mat4 {
        let c = radians.cos();
        let s = radians.sin();

        let mut r = Mat4::identity();
        r.set(0, 0, c);
        r.set(1, 0, s);
        r.set(0, 1, -s);
        r.set(1, 1, c);
        r
    }

    pub fn perspective(fov_y_radians: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        let f = 1.0 / (0.5 * fov_y_radians).tan();
        let nf = 1.0 / (near - far);

        let mut p = Mat4::identity();
        p.set(0, 0, f / aspect);
        p.set(1, 1, f);
        p.set(2, 2, (near + far) * nf);
        p.set(2, 3, (2.0 * far * near) * nf);
        p.set(3, 2, -1.0);
        p.set(3, 3, 0.0);
        p
    }

    pub fn look_at(eye: Vec3, centre: Vec3, up: Vec3) -> Mat4 {
        let f = Vec3::normalise(Vec3::sub(centre, eye));
        let s = Vec3::normalise(Vec3::cross(f, up));
        let u = Vec3::cross(s, f);
    
        let mut v = Mat4::identity();
        v.set(0, 0, s.x);
        v.set(1, 0, s.y);
        v.set(2, 0, s.z);
        v.set(0, 1, u.x);
        v.set(1, 1, u.y);
        v.set(2, 1, u.z);
        v.set(0, 2, -f.x);
        v.set(1, 2, -f.y);
        v.set(2, 2, -f.z);
        v.set(0, 3, -Vec3::dot(s, eye));
        v.set(1, 3, -Vec3::dot(u, eye));
        v.set(2, 3, Vec3::dot(f, eye));
        v
    }
}
use crate::math::Mat4;

pub struct MatrixStack {
    stack: Vec<Mat4>,
}

impl MatrixStack {
    pub fn new() -> Self {
        Self { stack: vec![Mat4::identity()] }
    }

    pub fn top(&self) -> Mat4 {
        *self.stack.last().unwrap()
    }

    pub fn push(&mut self) {
        let t = self.top();
        self.stack.push(t);
    }

    pub fn pop(&mut self) {
        self.stack.pop().expect("Matrix stack underflow");
        if self.stack.is_empty() {
            self.stack.push(Mat4::identity());
        }
    }

    pub fn apply(&mut self, m: Mat4) {
        let t = Mat4::mul(self.top(), m);
        *self.stack.last_mut().unwrap() = t;
    }
}
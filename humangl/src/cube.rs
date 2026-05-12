use std::ffi::CString;
use crate::math::Mat4;

pub struct UnitCube {
    pub vao: gl::types::GLuint,
    pub vbo: gl::types::GLuint,
    pub program: gl::types::GLuint,
    pub u_mvp: gl::types::GLint,
    pub u_colour: gl::types::GLint,
}

impl UnitCube {
    pub fn new(program: gl::types::GLuint) -> Self {
        let (vao, vbo) = create_unit_cube_vao_vbo();

        let u_mvp = unsafe {gl::GetUniformLocation(program, cstr("u_mvp").as_ptr())};
        let u_colour = unsafe {gl::GetUniformLocation(program, cstr("u_colour").as_ptr())};

        if u_mvp < 0 { panic!("Failed to get u_mvp uniform location"); }
        if u_colour < 0 { panic!("Failed to get u_colour uniform location"); }

        Self { vao, vbo, program, u_mvp, u_colour }
    }

    pub fn draw(&self, mvp: &Mat4, colour: [f32; 3]) {
        unsafe {
            //use shader program
            gl::UseProgram(self.program);
            //use transform matrix
            gl::UniformMatrix4fv(self.u_mvp, 1, gl::FALSE, mvp.as_ptr());
            //user colour
            gl::Uniform3f(self.u_colour, colour[0], colour[1], colour[2]);
            //use cube vertex layout
            gl::BindVertexArray(self.vao);
            //draw 36 vertices (6 sides * 2 triangles * 3 vertices)
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
        }
    }
}

fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn create_unit_cube_vao_vbo() -> (gl::types::GLuint, gl::types::GLuint) {
    //returns 108 as each vertex has 3 coordinates so 36 * 3
    let vertices: [f32; 108] = unit_cube_positions();
    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    (vao, vbo)
}

fn unit_cube_positions() -> [f32; 108] {
    let h = 0.5f32;

    fn quad(a: [f32; 3], b: [f32; 3], c: [f32; 3], d: [f32; 3], out: &mut Vec<f32>) {
        out.extend_from_slice(&a); out.extend_from_slice(&b); out.extend_from_slice(&c);
        out.extend_from_slice(&a); out.extend_from_slice(&c); out.extend_from_slice(&d);
    }

    let mut v: Vec<f32> = Vec::with_capacity(108);
    // +X (right face): x = +h
    quad([ h, -h, -h], [ h,  h, -h], [ h,  h,  h], [ h, -h,  h], &mut v);

    // -X (left face): x = -h
    quad([-h, -h,  h], [-h,  h,  h], [-h,  h, -h], [-h, -h, -h], &mut v);
    //+Y
    quad([-h, h, -h], [-h, h, h], [h, h, h], [h, h, -h], &mut v);
    //-Y
    quad([-h, -h, h], [-h, -h, -h], [h, -h, -h], [h, -h, h], &mut v);
    //+Z
    quad([-h, -h, h], [h, -h, h], [h, h, h], [-h, h, h], &mut v);
    //-Z
    quad([h, h, -h], [-h, h, -h], [-h, -h, -h], [h, -h, -h], &mut v);

    v.try_into().unwrap()
}


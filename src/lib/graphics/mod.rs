use gl::types::*;
use std::ffi::CString;
use std::fs::read_to_string;
use std::mem;
use std::ptr;
use std::str;

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

// Shder path
static VS_SRC_PATH: &'static str = "./shaders/triangle.vert";
static FS_SRC_PATH: &'static str = "./shaders/triangle.frag";

// Shader source
pub static VS_SRC: &'static str = &read_to_string(VS_SRC_PATH).unwrap();

pub static FS_SRC: &'static str = &read_to_string(FS_SRC_PATH).unwrap();

// ↓ This is a horrible mess! TODO: clean this shit up!
pub fn draw_triangle(program: GLuint) {
    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create vertex array object and copy vertex data
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            // ↓ V E R Y  U N S A F E ! ! ! TODO: replace with something safer
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW
        );

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(
            program, 0, CString::new("out_color").unwrap().as_ptr()
        );

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(
            program, CString::new("position").unwrap().as_ptr()
        );
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null()
        );
    }
}

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);

        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null char
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("GetShaderInfoLog not valid utf8"));
        }
    }
    shader
}

pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
        }
        program
    }
}

pub fn clean_up(program: GLuint, fs: GLuint, vs: GLuint) {
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        // gl::DeleteBuffers(1, vbo);
        // gl::DeleteVertexArrays(1, vao);
    }
}
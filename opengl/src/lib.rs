pub mod ffi {
    pub use opengl_sys::*;
}

pub fn get_viewport() -> [i32; 4] {
    let mut viewport = [0; 4];
    unsafe {
        ffi::glGetIntegerv(ffi::GL_VIEWPORT, &mut viewport as _);
    }
    viewport
}

pub fn draw_arrays(type_: u32, first: i32, count: i32) {
    unsafe {
        ffi::glDrawArrays(type_, first, count);
    }
}

pub fn gl_viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        ffi::glViewport(x, y, width, height);
    }
}

pub fn gl_enable(gl_enum: u32) {
    unsafe {
        ffi::glEnable(gl_enum);
    }
}

pub fn gl_clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        ffi::glClearColor(r, g, b, a);
    }
}

pub fn gl_clear(gl_enum: u32) {
    unsafe {
        ffi::glClear(gl_enum);
    }
}

pub struct Shader {
    id: u32,
}

pub struct CompiledShader(Shader);

impl Drop for CompiledShader {
    fn drop(&mut self) {
        unsafe {
            ffi::glDeleteShader(self.0.id);
        }
    }
}

impl Shader {
    pub fn new(gl_enum_type: u32) -> Self {
        unsafe {
            Self {
                id: ffi::glCreateShader(gl_enum_type),
            }
        }
    }

    pub fn source(self, src: &[&str]) -> Self {
        let c_src = src
            .iter()
            .map(|src| std::ffi::CString::new(src.as_bytes()).unwrap())
            .collect::<Vec<_>>();
        let c_src_borrow = c_src.iter().map(|v| v.as_ptr()).collect::<Vec<_>>();
        unsafe {
            ffi::glShaderSource(self.id, src.len() as _, c_src_borrow.as_ptr(), 0 as _);
        }
        self
    }

    pub fn compile(self) -> Result<CompiledShader, String> {
        unsafe {
            ffi::glCompileShader(self.id);

            let mut success: i32 = 0;
            ffi::glGetShaderiv(self.id, ffi::GL_COMPILE_STATUS, &mut success as *mut i32);

            if success == 0 {
                let mut info: [u8; 512] = [0; 512];
                ffi::glGetShaderInfoLog(self.id, 512, 0 as _, &mut info as *mut u8 as *mut i8);
                Err(std::ffi::CString::new(info).unwrap().into_string().unwrap())
            } else {
                Ok(CompiledShader(self))
            }
        }
    }
}

pub struct Program {
    id: u32,
    shaders: Vec<CompiledShader>,
}

impl Program {
    pub fn new() -> Self {
        unsafe {
            Self {
                id: ffi::glCreateProgram(),
                shaders: Vec::new(),
            }
        }
    }

    pub fn attach_shader(&mut self, shader: CompiledShader) {
        let shader_id = shader.0.id;
        self.shaders.push(shader);
        unsafe {
            ffi::glAttachShader(self.id, shader_id);
        }
    }

    pub fn link_program(&mut self) -> Result<(), String> {
        unsafe {
            ffi::glLinkProgram(self.id);

            let mut success: i32 = 0;
            ffi::glGetProgramiv(self.id, ffi::GL_LINK_STATUS, &mut success as _);
            if success == 0 {
                let mut info: [u8; 512] = [0; 512];
                ffi::glGetProgramInfoLog(self.id, 512, 0 as _, &mut info as *mut u8 as *mut i8);
                Err(std::ffi::CString::new(info).unwrap().into_string().unwrap())
            } else {
                Ok(())
            }
        }
    }

    pub fn use_program(&self) {
        unsafe {
            ffi::glUseProgram(self.id);
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<i32> {
        unsafe {
            let result = ffi::glGetUniformLocation(
                self.id,
                std::ffi::CString::new(name.as_bytes()).unwrap().as_ptr(),
            );
            if result < 0 { None } else { Some(result) }
        }
    }

    pub fn set_uniform_matrix4fv(&self, name: &str, transpose: bool, matrices: &[&[f32]]) {
        let matrices_flatten = matrices
            .iter()
            .map(|v| v.to_vec())
            .flatten()
            .collect::<Vec<f32>>();

        unsafe {
            ffi::glUniformMatrix4fv(
                self.get_uniform_location(name).expect("uniform exists"),
                matrices.len() as _,
                transpose as _,
                matrices_flatten.as_ptr(),
            );
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            ffi::glDeleteProgram(self.id);
        }
    }
}

pub struct VertexArray {
    id: u32,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            ffi::glBindVertexArray(0);
            ffi::glDeleteVertexArrays(1, &self.id);
        }
    }
}

impl VertexArray {
    pub fn new() -> Self {
        unsafe {
            let mut id: u32 = 0;
            ffi::glGenVertexArrays(1, &mut id as _);
            Self { id }
        }
    }

    pub fn bind(&self) {
        unsafe {
            ffi::glBindVertexArray(self.id);
        }
    }
}

pub struct Buffer {
    id: u32,
    target: u32,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            ffi::glBindBuffer(self.target, 0);
            ffi::glDeleteBuffers(1, &self.id);
        }
    }
}

impl Buffer {
    pub fn new(target: u32) -> Self {
        unsafe {
            let mut id: u32 = 0;
            ffi::glGenBuffers(1, &mut id as _);
            Self { id, target }
        }
    }

    pub fn bind(&self) {
        unsafe {
            ffi::glBindBuffer(self.target, self.id);
        }
    }

    pub fn data(&self, vertices: &[f32], usage: u32) {
        unsafe {
            ffi::glBufferData(
                self.target,
                (vertices.len() * size_of::<f32>()) as _,
                vertices.as_ptr() as _,
                usage,
            );
        }
    }

    pub fn enable_vertex_attrib_array(index: u32) {
        unsafe {
            ffi::glEnableVertexAttribArray(index);
        }
    }

    pub fn vertex_attrib_pointer(
        index: u32,
        size: i32,
        type_: u32,
        normalize: bool,
        stride: i32,
        offset: usize,
    ) {
        unsafe {
            ffi::glVertexAttribPointer(index, size, type_, normalize as u8, stride, offset as _);
        }
    }
}

pub struct Texture {
    id: u32,
    target: u32,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            ffi::glBindTexture(self.target, 0);
            ffi::glDeleteTextures(1, &self.id);
        }
    }
}

impl Texture {
    pub fn new(target: u32) -> Self {
        let mut id = 0;
        unsafe {
            ffi::glGenTextures(1, &mut id as _);
            Self { id, target }
        }
    }

    pub fn bind(&self) {
        unsafe {
            ffi::glBindTexture(self.target, self.id);
        }
    }

    pub fn image_2d(
        &self,
        level: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: *const u8,
    ) {
        unsafe {
            ffi::glTexImage2D(
                self.target,
                level,
                internal_format.try_into().unwrap(),
                width,
                height,
                0,
                format,
                type_,
                pixels as _,
            );
        }
    }

    pub fn generate_mipmap(&self) {
        unsafe {
            ffi::glGenerateMipmap(self.target);
        }
    }
}

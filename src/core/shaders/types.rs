use std::{ffi::CString, marker::PhantomData};

use crate::core::utils;

use super::errors::{get_shader_error, get_shader_program_error};

pub trait ShaderType 
{
    fn to_gl_enum() -> gl::types::GLuint;
}

#[derive(Clone, Debug)]
pub struct VertexShaderType {}
#[derive(Clone, Debug)]
pub struct FragmentShaderType {}

impl ShaderType for VertexShaderType 
{
    fn to_gl_enum() -> gl::types::GLuint 
    {
        return gl::VERTEX_SHADER;
    }
}

impl PartialEq for VertexShaderType
{
    fn eq(&self, _other: &Self) -> bool {
        return true;
    }
}

impl Eq for VertexShaderType {}

impl ShaderType for FragmentShaderType 
{
    fn to_gl_enum() -> gl::types::GLuint 
    {
        return gl::FRAGMENT_SHADER;
    }
}

impl PartialEq for FragmentShaderType
{
    fn eq(&self, _other: &Self) -> bool {
        return true;
    }
}

impl Eq for FragmentShaderType {}

#[derive(Debug)]
pub struct Shader<T: ShaderType> 
{
    id: gl::types::GLuint,
    shader_type: PhantomData<T>,
}

impl<T: ShaderType> Shader<T>
{
    pub fn load(source: &str) -> anyhow::Result<Self> 
    {
        let c_string = CString::new(source)?;
        
        let id = unsafe { gl::CreateShader(T::to_gl_enum()) };
        let mut success: gl::types::GLint = 1;

        unsafe 
        {
            gl::ShaderSource(id, 1, &c_string.as_ptr(), std::ptr::null());
            gl::CompileShader(id);

            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 
        {
            return Err(get_shader_error::<T>(id));
        }

        return Ok
        (
            Shader 
            {
                id,
                shader_type: PhantomData,
            }
        );
    }

    pub fn id(&self) -> gl::types::GLuint
    {
        return self.id;
    }
}

impl<T: ShaderType> Drop for Shader<T>
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram 
{
    id: gl::types::GLuint,
    vertex_shader: Shader<VertexShaderType>,
    fragment_shader: Shader<FragmentShaderType>,
}

impl ShaderProgram 
{
    pub fn load(vertex_shader: Shader<VertexShaderType>, fragment_shader: Shader<FragmentShaderType>) -> anyhow::Result<Self>
    {
        let id = unsafe { gl::CreateProgram() };
        let mut success: gl::types::GLint = 1;

        unsafe 
        {
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);
            gl::LinkProgram(id);

            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 
        {
            return Err(get_shader_program_error(id));
        }

        unsafe 
        {
            let mut n_attributes = 0;
            let mut n_uniforms = 0;
            gl::GetProgramInterfaceiv(id, gl::PROGRAM_INPUT, gl::ACTIVE_RESOURCES, &mut n_attributes);
            gl::GetProgramInterfaceiv(id, gl::UNIFORM, gl::ACTIVE_RESOURCES, &mut n_uniforms);

            for i in 0..n_attributes
            {
                let props = [gl::NAME_LENGTH, gl::TYPE, gl::ARRAY_SIZE];
                let mut params = [0, 0, 0];
                gl::GetProgramResourceiv(
                    id, 
                    gl::PROGRAM_INPUT, 
                    i as u32, 
                    props.len() as i32, 
                    props.as_ptr(), 
                    params.len() as i32, 
                    std::ptr::null_mut(), 
                    params.as_mut_ptr()
                );

                let mut buffer = utils::empty_cstring(params[0] as usize);
                gl::GetProgramResourceName(
                    id, 
                    gl::PROGRAM_INPUT, 
                    i as u32, 
                    params[0], 
                    std::ptr::null_mut(), 
                    buffer.as_ptr() as *mut gl::types::GLchar
                );

                let name = buffer.to_string_lossy().into_owned();
                println!("New program compiled!\n");
                println!("Inputs:");
                println!("Name: {name}");
                println!("\n");
            }

            for i in 0..n_uniforms
            {
                let props = [gl::NAME_LENGTH, gl::TYPE, gl::ARRAY_SIZE];
                let mut params = [0, 0, 0];
                gl::GetProgramResourceiv(
                    id, 
                    gl::UNIFORM, 
                    i as u32, 
                    props.len() as i32, 
                    props.as_ptr(), 
                    params.len() as i32, 
                    std::ptr::null_mut(), 
                    params.as_mut_ptr()
                );

                let mut buffer = utils::empty_cstring(params[0] as usize);
                gl::GetProgramResourceName(
                    id, 
                    gl::UNIFORM, 
                    i as u32, 
                    params[0], 
                    std::ptr::null_mut(), 
                    buffer.as_ptr() as *mut gl::types::GLchar
                );

                let name = buffer.to_string_lossy().into_owned();
                println!("Uniforms:");
                println!("Name: {name}");
                println!("\n");
            }
        }

        return Ok
        (
            ShaderProgram
            {
                id,
                vertex_shader,
                fragment_shader,
            }
        );
    }

    pub fn id(&self) -> gl::types::GLuint
    {
        return self.id;
    }

    pub fn activate(&self)
    {
        unsafe {
            gl::UseProgram(self.id)
        }
    }
}

impl Drop for ShaderProgram
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
use thiserror::Error;
use anyhow::anyhow;

use crate::core::utils::empty_cstring;

use super::types::ShaderType;

#[derive(Clone, Debug, Error)]
pub enum ShaderError {
    #[error("Failed to load vertex shader: {0}")]
    LoadVertexShaderError(String),
    #[error("Failed to load fragment shader: {0}")]
    LoadFragmentShaderError(String),   
    #[error("Unknown shader type: {0}")]
    UnknownShaderError(gl::types::GLuint),
}

#[derive(Clone, Debug, Error)]
pub enum ShaderProgramError {
    #[error("Failed to link shaders as program: {0}")]
    LinkShaderProgramError(String),
}

pub fn get_shader_error<T: ShaderType>(id: gl::types::GLuint) -> anyhow::Error
{
    let mut len: gl::types::GLint = 0;

    unsafe 
    {
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
    }

    let error = empty_cstring(len as usize);

    unsafe
    {
        gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
    }

    let message = error.to_string_lossy().into_owned();

    match T::to_gl_enum() {
        gl::VERTEX_SHADER => return anyhow!(ShaderError::LoadVertexShaderError(message)),
        gl::FRAGMENT_SHADER => return anyhow!(ShaderError::LoadFragmentShaderError(message)),
        _unknown => return anyhow!(ShaderError::UnknownShaderError(_unknown)),
    };
}

pub fn get_shader_program_error(id: gl::types::GLuint) -> anyhow::Error
{
    let mut len: gl::types::GLint = 0;

    unsafe 
    {
        gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
    }

    let error = empty_cstring(len as usize);

    unsafe
    {
        gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
    }

    let message = error.to_string_lossy().into_owned();

    return anyhow!(ShaderProgramError::LinkShaderProgramError(message))
}
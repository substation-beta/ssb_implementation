// Imports
use super::safe::*;

#[inline]
pub fn build_program(vshader_source: &str, fshader_source: &str) -> Program {
    // Compile shaders
    let vertex_shader = Shader::create(gl32::VERTEX_SHADER);
    vertex_shader.source(vshader_source);
    vertex_shader.compile().expect("Vertex shader couldn't compile!");
    let fragment_shader = Shader::create(gl32::FRAGMENT_SHADER);
    fragment_shader.source(fshader_source);
    fragment_shader.compile().expect("Fragment shader couldn't compile!");
    // Link shader program
    let shader_program = Program::create();
    shader_program.attach(&vertex_shader);
    shader_program.attach(&fragment_shader);
    shader_program.link().expect("Shader program couldn't link!");
    // Return only program, shaders aren't needed anymore
    shader_program
}
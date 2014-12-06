/*
 * Copyright (c) 2012-2013 Graham Sellers
 * Copyright (c) 2014 Cameron Hart
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#![feature(globs)]

extern crate gl;
extern crate sb6;

use gl::types::*;
use std::ptr;

const VS_SRC: &'static str = "\
#version 330 core                                                              \n\
                                                                               \n\
void main(void)                                                                \n\
{                                                                              \n\
    const vec4 vertices[] = vec4[](vec4(-1.0, -1.0, 0.5, 1.0),                 \n\
                                   vec4( 1.0, -1.0, 0.5, 1.0),                 \n\
                                   vec4(-1.0,  1.0, 0.5, 1.0),                 \n\
                                   vec4( 1.0,  1.0, 0.5, 1.0));                \n\
                                                                               \n\
    gl_Position = vertices[gl_VertexID];                                       \n\
}                                                                              \n\
";

const FS_SRC: &'static str = "\
#version 330 core                                                              \n\
                                                                               \n\
uniform sampler2D s;                                                           \n\
out vec4 color;                                                                \n\
                                                                               \n\
void main(void)                                                                \n\
{                                                                              \n\
    color = texture(s, gl_FragCoord.xy / textureSize(s, 0));                   \n\
}                                                                              \n\
";

struct MyApp {
    info: sb6::AppInfo,
    texture: GLuint,
    program: GLuint,
    vao: GLuint
}

impl MyApp {
    fn new(init: sb6::AppInfo) -> MyApp {
        MyApp {
            info: init,
            texture: 0,
            program: 0,
            vao: 0
        }
    }
}

impl sb6::App for MyApp {
    fn get_app_info(&self) -> &sb6::AppInfo { &self.info }
    fn startup(&mut self) {
        unsafe {
            // Load texture from file
            self.texture = match sb6::ktx::load("media/textures/Tree.ktx") {
                Ok(v) => v,
                Err(e) => panic!("failed to load: {}", e)
            };
            self.program = gl::CreateProgram();

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            FS_SRC.with_c_str(
                |ptr| gl::ShaderSource(fs, 1, &ptr, ptr::null()));
            gl::CompileShader(fs);
            sb6::shader::check_compile_status(fs).unwrap();

            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            VS_SRC.with_c_str(
                |ptr| gl::ShaderSource(vs, 1, &ptr, ptr::null()));
            gl::CompileShader(vs);
            sb6::shader::check_compile_status(vs).unwrap();

            gl::AttachShader(self.program, vs);
            gl::AttachShader(self.program, fs);
            gl::LinkProgram(self.program);
            sb6::program::check_link_status(self.program).unwrap();

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }
    }

    fn shutdown(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteProgram(self.program);
            gl::DeleteTextures(1, &self.texture);
        }
        self.vao = 0;
        self.program = 0;
        self.texture = 0;
    }

    fn render(&self, _: f64) {
        const GREEN: [GLfloat, ..4] = [ 0.0, 0.25, 0.0, 1.0 ];

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GREEN.as_ptr());
            gl::UseProgram(self.program);
            gl::Viewport(0, 0, self.info.window_width as i32,
                         self.info.window_height as i32);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }
}

fn main() {
    let mut init = sb6::AppInfo::default();
    init.title = "OpenGL SuperBible - KTX Viewer";
    init.major_version = 3;
    init.minor_version = 3;
    let mut app = MyApp::new(init);
    sb6::run(&mut app);
}


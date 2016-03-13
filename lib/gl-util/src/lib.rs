//! Utility wrappers to simplify writing OpenGL code.
//!
//! This crate aspires to provide an abstraction over OpenGL's raw API in order to simplify the
//! task of writing higher-level rendering code for OpenGL. `gl-util` is much in the vein of
//! [glutin](https://github.com/tomaka/glium) and [gfx-rs](https://github.com/gfx-rs/gfx) before
//! it, the main difference being that it is much more poorly constructed and is being developed by
//! someone much less experienced with OpenGL.

extern crate bootstrap_gl as gl;

use gl::{
    BufferName, BufferTarget, BufferUsage, ClearBufferMask, GlType, IndexType, ProgramObject,
    ServerCapability, VertexArrayName
};
use std::mem;

pub use gl::{
    AttributeLocation, Comparison, DrawMode, Face, PolygonMode, ShaderType, WindingOrder,
};
pub use gl::platform::swap_buffers;
pub use self::shader::*;

pub mod shader;

/// Initializes global OpenGL state and creates the OpenGL context needed to perform rendering.
pub fn init() {
    gl::create_context();
}

/// TODO: Take clear mask (and values) as parameters.
pub fn clear() {
    unsafe { gl::clear(ClearBufferMask::Color | ClearBufferMask::Depth); }
}

/// Represents a buffer of vertex data and the layout of that data.
///
/// Wraps a vertex buffer object and vertex array object into one struct.
#[derive(Debug)]
pub struct VertexBuffer {
    buffer_name: BufferName,
    vertex_array_name: VertexArrayName,
    len: usize,
    element_len: usize,
}

impl VertexBuffer {
    /// Creates a new `VertexBuffer` object.
    pub fn new() -> VertexBuffer {
        let mut buffer_name = BufferName::null();
        let mut vertex_array_name = VertexArrayName::null();
        unsafe {
            gl::gen_buffers(1, &mut buffer_name);
            gl::gen_vertex_arrays(1, &mut vertex_array_name);
        }

        VertexBuffer {
            buffer_name: buffer_name,
            vertex_array_name: vertex_array_name,
            len: 0,
            element_len: 0,
        }
    }

    /// Fills the buffer with the contents of the data slice.
    pub fn set_data_f32(&mut self, data: &[f32]) {
        self.len = data.len();

        let data_ptr = data.as_ptr() as *const ();
        let byte_count = data.len() * mem::size_of::<f32>();

        unsafe {
            gl::bind_buffer(BufferTarget::Array, self.buffer_name);
            gl::buffer_data(
                BufferTarget::Array,
                byte_count as isize,
                data_ptr,
                BufferUsage::StaticDraw);
            gl::bind_buffer(BufferTarget::Array, BufferName::null());
        }
    }

    /// Specifies how the data for a particular vertex attribute is laid out in the buffer.
    ///
    /// # Parameters
    ///
    /// - `attrib` - The attribute being set. This can be gotten using `Program::get_attrib()`.
    /// - `elements` - The number of `f32`s per vertex for this attribute. For example, for a
    ///   vertex normal with x, y, and z coordinates `elements` would be 3.
    /// - `stride` - The offset in elements between consecutive vertex attributes. If `stride` is
    ///   attributes are understood to be tightly packed.
    /// - `offset` - The offset in elements from the start of the buffer where the first attribute
    ///   is located.
    pub fn set_attrib_f32(
        &mut self,
        attrib: AttributeLocation,
        elements: usize,
        stride: usize,
        offset: usize
    ) {
        // Calculate the number of elements based on the attribute.
        // TODO: Verify that each attrib has the same element length.
        self.element_len = (self.len - offset) / elements + stride;

        unsafe {
            gl::bind_buffer(BufferTarget::Array, self.buffer_name);
            gl::bind_vertex_array(self.vertex_array_name);

            gl::enable_vertex_attrib_array(attrib);
            gl::vertex_attrib_pointer(
                attrib,
                elements as i32,
                GlType::Float,
                false,
                (stride * mem::size_of::<f32>()) as i32,
                offset * mem::size_of::<f32>());

            gl::bind_vertex_array(VertexArrayName::null());
            gl::bind_buffer(BufferTarget::Array, BufferName::null());
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::delete_buffers(1, &mut self.buffer_name);
            gl::delete_vertex_arrays(1, &mut self.vertex_array_name);
        }
    }
}

#[derive(Debug)]
pub struct IndexBuffer {
    buffer_name: BufferName,
    len: usize,
}

impl IndexBuffer {
    // Create a new index buffer.
    pub fn new() -> IndexBuffer {
        let mut buffer_name = BufferName::null();
        unsafe {
            gl::gen_buffers(1, &mut buffer_name);
        }

        IndexBuffer {
            buffer_name: buffer_name,
            len: 0,
        }
    }

    pub fn set_data_u32(&mut self, data: &[u32]) {
        self.len = data.len();

        let data_ptr = data.as_ptr() as *const ();
        let byte_count = data.len() * mem::size_of::<u32>();

        unsafe {
            gl::bind_buffer(BufferTarget::ElementArray, self.buffer_name);
            gl::buffer_data(
                BufferTarget::ElementArray,
                byte_count as isize,
                data_ptr,
                BufferUsage::StaticDraw);
            gl::bind_buffer(BufferTarget::ElementArray, BufferName::null());
        }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::delete_buffers(1, &mut self.buffer_name);
        }
    }
}

/// A configuration object for specifying all of the various configurable options for a draw call.
pub struct DrawBuilder<'a> {
    vertex_buffer: &'a VertexBuffer,
    draw_mode: DrawMode,
    index_buffer: Option<&'a IndexBuffer>,
    polygon_mode: Option<PolygonMode>,
    program: Option<&'a Program>,
    cull: Option<Face>,
    depth_test: Option<Comparison>,
    winding_order: Option<WindingOrder>,
}

impl<'a> DrawBuilder<'a> {
    pub fn new(vertex_buffer: &VertexBuffer, draw_mode: DrawMode) -> DrawBuilder {
        DrawBuilder {
            vertex_buffer: vertex_buffer,
            draw_mode: draw_mode,
            index_buffer: None,
            polygon_mode: None,
            program: None,
            cull: None,
            depth_test: None,
            winding_order: None,
        }
    }

    pub fn index_buffer(&'a mut self, index_buffer: &'a IndexBuffer) -> &mut DrawBuilder {
        self.index_buffer = Some(index_buffer);
        self
    }

    pub fn polygon_mode(&'a mut self, polygon_mode: PolygonMode) -> &mut DrawBuilder {
        self.polygon_mode = Some(polygon_mode);
        self
    }

    pub fn program(&'a mut self, program: &'a Program) -> &mut DrawBuilder {
        self.program = Some(program);
        self
    }

    pub fn cull(&'a mut self, face: Face) -> &mut DrawBuilder {
        self.cull = Some(face);
        self
    }

    pub fn depth_test(&'a mut self, comparison: Comparison) -> &mut DrawBuilder {
        self.depth_test = Some(comparison);
        self
    }

    pub fn winding(&'a mut self, winding_order: WindingOrder) -> &mut DrawBuilder {
        self.winding_order = Some(winding_order);
        self
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::bind_vertex_array(self.vertex_buffer.vertex_array_name);
            gl::bind_buffer(BufferTarget::Array, self.vertex_buffer.buffer_name);

            if let Some(polygon_mode) = self.polygon_mode {
                gl::polygon_mode(Face::FrontAndBack, polygon_mode);
            }

            if let Some(program) = self.program {
                let Program(program_object) = *program;
                gl::use_program(program_object);
            }

            if let Some(face) = self.cull {
                gl::enable(ServerCapability::CullFace);
                gl::cull_face(face);

                if let Some(winding_order) = self.winding_order {
                    gl::front_face(winding_order);
                }
            }

            if let Some(depth_test) = self.depth_test {
                gl::enable(ServerCapability::DepthTest);
                gl::depth_func(depth_test);
            }

            if let Some(indices) = self.index_buffer {
                gl::bind_buffer(BufferTarget::ElementArray, indices.buffer_name);
                gl::draw_elements(
                    self.draw_mode,
                    indices.len as i32,
                    IndexType::UnsignedInt,
                    0);
            } else {
                gl::draw_arrays(
                    self.draw_mode,
                    0,
                    self.vertex_buffer.element_len as i32);
            }

            // Reset all values even if they weren't used so that we don't need to branch twice on
            // each option.
            gl::front_face(WindingOrder::CounterClockwise);
            gl::disable(ServerCapability::DepthTest);
            gl::disable(ServerCapability::CullFace);
            gl::polygon_mode(Face::FrontAndBack, PolygonMode::Fill);
            gl::use_program(ProgramObject::null());
            gl::bind_buffer(BufferTarget::ElementArray, BufferName::null());
            gl::bind_buffer(BufferTarget::Array, BufferName::null());
            gl::bind_vertex_array(VertexArrayName::null());
        }
    }
}

/// Represents a complete shader program which can be used in rendering.
#[derive(Debug, Clone)]
pub struct Program(ProgramObject);

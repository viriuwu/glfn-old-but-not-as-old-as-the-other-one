//! OpenGL XML Registry constants

// Used to prevent rustdoc from embedding the like 2.5MB XMLs
use std::convert::identity;

/// XML Registry for OpenGL and OpenGL ES
pub const GL: &str = identity(include_str!("../khronos/gl.xml"));

/// XML Registry for GLX (OpenGL Extension to the X Window System)
pub const GLX: &str = identity(include_str!("../khronos/glx.xml"));

/// XML Registry for WGL (OpenGL Extension for Windows)
pub const WGL: &str = identity(include_str!("../khronos/wgl.xml"));

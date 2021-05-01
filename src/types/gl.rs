use crate::{
    types::{Mutability::{Const, Mut}, Repr::{Int, Uint, Float}, Type, Typedef},
    Platform
};
use std::borrow::Cow::Borrowed as B;

const GL_XML_TYPES: &[Type] = &[
    Type { name: B("GLenum"),        def: Typedef::Alias  { target: B("c_uint")   }, ptr: None },
    Type { name: B("GLboolean"),     def: Typedef::Alias  { target: B("c_uchar")  }, ptr: None },
    Type { name: B("GLbitfield"),    def: Typedef::Alias  { target: B("c_uint")   }, ptr: None },

    Type { name: B("GLvoid"),        def: Typedef::Alias  { target: B("c_void")   }, ptr: None },
    Type { name: B("GLbyte"),        def: Typedef::Fixed  { bits:  8, repr: Int   }, ptr: None },
    Type { name: B("GLubyte"),       def: Typedef::Fixed  { bits:  8, repr: Uint  }, ptr: None },
    Type { name: B("GLshort"),       def: Typedef::Fixed  { bits: 16, repr: Int   }, ptr: None },
    Type { name: B("GLushort"),      def: Typedef::Fixed  { bits: 16, repr: Uint  }, ptr: None },
    Type { name: B("GLint"),         def: Typedef::Alias  { target: B("c_int")    }, ptr: None },
    Type { name: B("GLuint"),        def: Typedef::Alias  { target: B("c_uint")   }, ptr: None },
    Type { name: B("GLclampx"),      def: Typedef::Fixed  { bits: 32, repr: Int   }, ptr: None },
    Type { name: B("GLfixed"),       def: Typedef::Fixed  { bits: 32, repr: Int   }, ptr: None },
    Type { name: B("GLsizei"),       def: Typedef::Alias  { target: B("c_int")    }, ptr: None },
    Type { name: B("GLfloat"),       def: Typedef::Fixed  { bits: 32, repr: Float }, ptr: None },
    Type { name: B("GLclampf"),      def: Typedef::Fixed  { bits: 32, repr: Float }, ptr: None },
    Type { name: B("GLdouble"),      def: Typedef::Alias  { target: B("c_double") }, ptr: None },
    Type { name: B("GLclampd"),      def: Typedef::Alias  { target: B("c_double") }, ptr: None },
    Type { name: B("GLint64"),       def: Typedef::Fixed  { bits: 64, repr: Int   }, ptr: None },
    Type { name: B("GLuint64"),      def: Typedef::Fixed  { bits: 64, repr: Uint  }, ptr: None },

    Type { name: B("GLchar"),        def: Typedef::Alias  { target: B("c_char")   }, ptr: None },
    Type { name: B("GLcharARB"),     def: Typedef::Alias  { target: B("c_char")   }, ptr: None },
    Type { name: B("GLhalf"),        def: Typedef::Fixed  { bits: 16, repr: Uint  }, ptr: None },
    Type { name: B("GLhalfNV"),      def: Typedef::Alias  { target: B("c_ushort") }, ptr: None },
    Type { name: B("GLhalfARB"),     def: Typedef::Fixed  { bits: 16, repr: Uint  }, ptr: None },
    Type { name: B("GLintptr"),      def: Typedef::IntPtr { signed: true          }, ptr: None },
    Type { name: B("GLsizeiptr"),    def: Typedef::IntPtr { signed: true          }, ptr: None },
    Type { name: B("GLsizeiptrARB"), def: Typedef::IntPtr { signed: true          }, ptr: None },
    Type { name: B("GLint64EXT"),    def: Typedef::Fixed  { bits: 64, repr: Int   }, ptr: None },
    Type { name: B("GLuint64EXT"),   def: Typedef::Fixed  { bits: 64, repr: Uint  }, ptr: None },

    Type { name: B("GLvdpauSurfaceNV"),     def: Typedef::Alias  { target: B("GLintptr")  }, ptr: None      },
    Type { name: B("GLeglClientBufferEXT"), def: Typedef::Alias  { target: B("c_void")    }, ptr: Some(Mut) },
    Type { name: B("GLeglImageOES"),        def: Typedef::Alias  { target: B("c_void")    }, ptr: Some(Mut) },
    Type { name: B("GLsync"),               def: Typedef::Opaque { target: B("__GLsync")  }, ptr: Some(Mut) },
    Type { name: B("_cl_context"),          def: Typedef::Unit                             , ptr: None      },
    Type { name: B("_cl_event"),            def: Typedef::Unit                             , ptr: None      },

    Type { name: B("GLDEBUGPROC"), def: Typedef::Function {
        ret: B("c_void"),
        args: B(&[
            Type { name: B("source"),    def: Typedef::Alias { target: B("GLenum")  }, ptr: None        },
            Type { name: B("type"),      def: Typedef::Alias { target: B("GLenum")  }, ptr: None        },
            Type { name: B("id"),        def: Typedef::Alias { target: B("GLuint")  }, ptr: None        },
            Type { name: B("severity"),  def: Typedef::Alias { target: B("GLenum")  }, ptr: None        },
            Type { name: B("length"),    def: Typedef::Alias { target: B("GLsizei") }, ptr: None        },
            Type { name: B("message"),   def: Typedef::Alias { target: B("GLchar")  }, ptr: Some(Const) },
            Type { name: B("userParam"), def: Typedef::Alias { target: B("c_void")  }, ptr: Some(Const) },
        ]),
    }, ptr: None },
    Type { name: B("GLDEBUGPROCAMD"), def: Typedef::Function {
        ret: B("c_void"),
        args: B(&[
            Type { name: B("id"),        def: Typedef::Alias { target: B("GLuint")  }, ptr: None        },
            Type { name: B("category"),  def: Typedef::Alias { target: B("GLenum")  }, ptr: None        },
            Type { name: B("severity"),  def: Typedef::Alias { target: B("GLenum")  }, ptr: None        },
            Type { name: B("length"),    def: Typedef::Alias { target: B("GLsizei") }, ptr: None        },
            Type { name: B("message"),   def: Typedef::Alias { target: B("GLchar")  }, ptr: Some(Const) },
            Type { name: B("userParam"), def: Typedef::Alias { target: B("c_void")  }, ptr: Some(Mut)   },
        ]),
    }, ptr: None },
    Type { name: B("GLDEBUGPROCARB"), def: Typedef::Alias    { target: B("GLDEBUGPROC")       }, ptr: None      },
    Type { name: B("GLDEBUGPROCKHR"), def: Typedef::Alias    { target: B("GLDEBUGPROC")       }, ptr: None      },
    Type { name: B("GLVULKANPROCNV"), def: Typedef::Function { ret: B("c_void"), args: B(&[]) }, ptr: Some(Mut) },
];

pub fn get(platform: Platform) -> Vec<Type> {
    let mut types = Vec::with_capacity(GL_XML_TYPES.len() + 1);
    types.extend_from_slice(GL_XML_TYPES);
    match platform {
        Platform::Mac => types.push(Type {
            name: B("GLhandleARB"),
            def: Typedef::Alias { target: B("c_void") },
            ptr: Some(Mut),
        }),
        _ => types.push(Type {
            name: B("GLhandleARB"),
            def: Typedef::Alias { target: B("c_uint") },
            ptr: None,
        }),
    }
    types
}

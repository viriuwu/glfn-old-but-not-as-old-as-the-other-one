mod gl;

use std::borrow::Cow;

#[derive(Clone)]
pub struct Type {
    name: Cow<'static, str>,
    def: Typedef,
    ptr: Option<Mutability>,
}

#[derive(Clone)]
pub enum Typedef {
    Alias {
        target: Cow<'static, str>,
    },
    Fixed {
        bits: u8,
        repr: Repr,
    },
    Function {
        ret: Cow<'static, str>,
        args: Cow<'static, [Type]>,
    },
    IntPtr {
        signed: bool,
    },
    Opaque {
        target: Cow<'static, str>,
    },
    Unit,
}

#[derive(Copy, Clone)]
pub enum Repr {
    Int,
    Uint,
    Float,
}

#[derive(Copy, Clone)]
pub enum Mutability {
    Const,
    Mut,
}

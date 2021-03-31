pub mod math;
pub use math::*;

pub mod matrix4;
pub use matrix4::*;

pub mod vertex;
pub use vertex::*;

pub mod mesh;
pub use mesh::*;

#[macro_use]
pub mod heap_array;
pub use heap_array::*;

pub mod bucket_array;
pub use bucket_array::*;

pub type Maybe<T = ()> = Result<T, Box<dyn std::error::Error>>;
pub type Meshes = std::collections::HashMap<u16, Mesh>;

#[allow(dead_code)]
pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];


#[derive(Default)]
pub struct GameState {
    pub should_quit: bool,
    pub camera:      Matrix4,
    pub t:           f32,
    pub position:    Vector3
}

#[derive(Default, Debug)]
pub struct Input {
    pub mouse: Vector2,
    pub scroll: Vector2,
    pub controller: Option<Controller>
}

#[derive(Default, Debug)]
pub struct Controller {
    pub buttons:     ButtonSet, // NOTE: its a u16
    pub left_stick:  Vector2,
    pub right_stick: Vector2,
}

#[repr(u16)]
pub enum Button {
    LeftTrigger  = 0x0001,
    RightTrigger = 0x0002,
    LeftBumper   = 0x0004,
    RightBumper  = 0x0008,
    
    FaceUp       = 0x0010,
    FaceLeft     = 0x0020,
    FaceDown     = 0x0040,
    FaceRight    = 0x0080,
    
    Select       = 0x0100,
    RightStick   = 0x0200,
    LeftStick    = 0x0400,
    Start        = 0x0800,
    
    DPadUp       = 0x1000,
    DPadRight    = 0x2000,
    DPadDown     = 0x4000,
    DPadLeft     = 0x8000,
}

pub type ButtonSet = u16;

impl std::ops::BitAnd<ButtonSet> for Button {
    type Output = bool;

    // rhs is the "right-hand side" of the expression `a & b`
    #[inline] fn bitand(self, rhs: ButtonSet) -> Self::Output {
        ((self as ButtonSet) & rhs) != 0
    }
}







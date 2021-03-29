use crate::types::*;
pub struct GameState {
    should_quit: bool,
    t: f32,
}

impl GameState {
    pub fn init() -> Self {
        GameState{should_quit: false, t: 0.0}
    }
}

pub fn draw_cube(id: u16, x: f32, y: f32, z: f32) -> Mesh {
    let vertices : Vec<Vertex> = vec! [
        Vertex { position: [-1.0, -1.0,  1.0, 1.0], uv: [0.0, 0.0, 1.0] },
        Vertex { position: [-1.0,  1.0,  1.0, 1.0], uv: [1.0, 0.0, 0.0] },
        Vertex { position: [ 1.0,  1.0,  1.0, 1.0], uv: [0.0, 1.0, 0.0] },
        Vertex { position: [ 1.0, -1.0,  1.0, 1.0], uv: [1.0, 1.0, 0.0] },
        Vertex { position: [-1.0, -1.0, -1.0, 1.0], uv: [1.0, 1.0, 1.0] },
        Vertex { position: [-1.0,  1.0, -1.0, 1.0], uv: [1.0, 0.0, 0.0] },
        Vertex { position: [ 1.0,  1.0, -1.0, 1.0], uv: [1.0, 0.0, 1.0] },
        Vertex { position: [ 1.0, -1.0, -1.0, 1.0], uv: [0.0, 0.0, 1.0] },
    ];
    
    let indices = vec! [
        0,2,1,  0,3,2,
        4,3,0,  4,7,3,
        4,1,5,  4,0,1,
        3,6,2,  3,7,6,
        1,6,5,  1,2,6,
        7,5,6,  7,4,5
    ];
    
    let model_matrix = {
        let mut m = Matrix4::identity();
        m.scale(0.9, 0.9, 0.9);
        m.translate(x, y, z);
        m
    };
    
    Mesh {vertices, indices, model_matrix, id}
}

pub fn game_update_and_render(state: &mut GameState,
                              meshes: &mut Meshes){
    let first = {
        let mut cube = draw_cube(0, -1.0, -1.0, 0.0);
        cube.model_matrix.rotate_about_x(state.t);
        cube.model_matrix.scale(0.5, 0.5, 0.5);
        cube
    };
    let second = {
        let mut cube = draw_cube(1,  1.0, 0.0, 0.0);
        cube.model_matrix.rotate_about_y(-state.t);
        cube.model_matrix.scale(0.5, 0.5, 0.5);
        cube
    };
    
    meshes.insert(first.id, first);
    meshes.insert(second.id, second);


    state.t += 0.008;

}

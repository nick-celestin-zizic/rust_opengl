use crate::types::*;

#[derive(Default)]
pub struct GameState {
    pub should_quit: bool,
    pub camera:      Matrix4,
    pub t:           f32,
}

#[derive(Default, Debug)]
pub struct Input {
    pub mouse: Vector2
}

pub fn draw_cube(id: u16, x: f32, y: f32, z: f32) -> Mesh {
    let vertices : Vec<Vertex> = vec! [
        Vertex { position: [-1.0, -1.0,  1.0, 1.0],
                 color:    [ 0.0,  0.0,  1.0, 1.0], .. Default::default() },
        Vertex { position: [-1.0,  1.0,  1.0, 1.0],
                 color:    [ 1.0,  0.0,  0.0, 1.0], .. Default::default() },
        Vertex { position: [ 1.0,  1.0,  1.0, 1.0],
                 color:    [ 0.0,  1.0,  0.0, 1.0], .. Default::default() },
        Vertex { position: [ 1.0, -1.0,  1.0, 1.0],
                 color:    [ 1.0,  1.0,  0.0, 1.0], .. Default::default() },
        Vertex { position: [-1.0, -1.0, -1.0, 1.0],
                 color:    [ 1.0,  1.0,  1.0, 1.0], .. Default::default() },
        Vertex { position: [-1.0,  1.0, -1.0, 1.0],
                 color:    [ 1.0,  0.0,  0.0, 1.0], .. Default::default() },
        Vertex { position: [ 1.0,  1.0, -1.0, 1.0],
                 color:    [ 1.0,  0.0,  1.0, 1.0], .. Default::default() },
        Vertex { position: [ 1.0, -1.0, -1.0, 1.0],
                 color:    [ 0.0,  0.0,  1.0, 1.0], .. Default::default() },
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

pub fn game_update_and_render(state:  &mut GameState,
                              meshes: &mut Meshes,
                              input:  &mut Input){
    let mut id_gen = 0..;
    let first = {
        let mut cube = draw_cube(id_gen.next().unwrap(), -1.0, -1.0, 0.0);
        cube.model_matrix.rotate_about_x(state.t);
        cube.model_matrix.scale(0.05, 0.05, 0.05);
        cube.model_matrix.translate(input.mouse[0],
                                    -input.mouse[1], -1.1);
        cube
    };

    let second = {
        let mut cube = draw_cube(id_gen.next().unwrap(),  1.0, 0.0, 0.0);
        cube.model_matrix.rotate_about_y(-state.t);
        cube.model_matrix.scale(0.5, 0.5, 0.5);
        cube.model_matrix.translate(0.0, 0.0, -5.0);
        cube
    };
    
    meshes.insert(first.id, first);
    meshes.insert(second.id, second);

    state.t += 0.008;
}

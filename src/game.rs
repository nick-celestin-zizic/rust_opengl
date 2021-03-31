use crate::types::*;

pub fn game_update_and_render(state:  &mut GameState,
                              meshes: &mut Meshes,
                              input:  &mut Input) {
    let mut id_gen = 0..;

    if let Some(controller) = &input.controller {
        if Button::FaceDown & controller.buttons {
            println!("WAHOOO");
        }

        println!("{:?}", controller.left_stick);

        state.position[0] += controller.left_stick[0];
        state.position[1] += -controller.left_stick[1];
        state.position[2] += controller.right_stick[1];
    }
    
    let first = {
        let mut cube = draw_cube(id_gen.next().unwrap(), -1.0, -1.0, 0.0);
        cube.model_matrix.rotate_about_x(state.t);
        cube.model_matrix.scale(0.5, 0.5, 0.5);
        cube.model_matrix.translate(
            input.mouse[0], -input.mouse[1], -1.1 + input.scroll[1] * 0.01);
        cube
    };

    let second = {
        let mut cube = draw_cube(id_gen.next().unwrap(),  1.0, 0.0, 0.0);
        cube.model_matrix.rotate_about_y(-state.t);
        cube.model_matrix.scale(0.5, 0.5, 0.5);
        cube.model_matrix.translate(
            state.position[0], state.position[1], state.position[2]);
        cube
    };
    
    meshes.insert(first.id, first);
    meshes.insert(second.id, second);

    state.t += 0.08;
}

pub fn draw_cube(id: u16, x: f32, y: f32, z: f32) -> Mesh {
    let vertices = vec! [
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
        m.translate(x, y, z);
        m
    };
    
    Mesh {vertices, indices, model_matrix, id}
}

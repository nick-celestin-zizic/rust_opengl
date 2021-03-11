use crate::types::*;

pub fn game_update_and_render(meshes : &mut Vec<Mesh>){
    meshes.push(Mesh::load("box").unwrap());
    meshes.push(Mesh::load("teapot").unwrap());
}

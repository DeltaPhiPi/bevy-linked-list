use bevy::render::{color::Color, mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues}, render_asset::RenderAssetUsages};
fn rectangle_vertices(width: f32, height: f32) -> Vec<[f32; 2]> {
    let w = width / 2.0;
    let h = height / 2.0;
    vec![
        [-w, -h],
        [-w, h],
        [w, -h],
        [w, h],
    ]
}
pub fn rectangle_mesh(width: f32, height: f32, border_size: f32, fill: Color, border: Color) -> Mesh {
    let mut rect = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    let mut vert = rectangle_vertices(width, height);
    vert.extend(rectangle_vertices(width-border_size, height-border_size));
    vert.extend(rectangle_vertices(width-border_size, height-border_size));
    rect.insert_attribute(Mesh::ATTRIBUTE_POSITION, vert.into_iter().map(|x| [x[0], x[1], 0.0]).collect::<Vec<_>>());
    rect.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![
        border,border,border,border,
        border,border,border,border,
        fill,fill,fill,fill,
    ].into_iter().map(|x| x.as_linear_rgba_f32()).collect::<Vec<_>>());
    rect.insert_indices(Indices::U32(vec![
        0,1,5,
        0,5,4,
        0,4,6,
        0,6,2,
        3,5,1,
        3,7,5,
        3,6,7,
        3,2,6,
        
        8,9,10,
        11,9,10,
    ]));
    rect
}

pub fn cross_mesh(fill: Color, border: Color, cross: Color) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    let mut vert = rectangle_vertices(1.0, 1.0);
    vert.extend(rectangle_vertices(0.95, 0.95));
    vert.extend(rectangle_vertices(0.95, 0.95));
    vert.extend_from_slice(&[
        [-0.3, -0.1],
        [-0.3, 0.1],
        [-0.1, 0.1],
        [-0.1, 0.3],
        [0.1, 0.3],
        [0.1, 0.1],
        [0.3, 0.1],
        [0.3, -0.1],
        [0.1, -0.1],
        [0.1, -0.3],
        [-0.1, -0.3],
        [-0.1, -0.1]
    ]);
    vert.extend_from_slice(&[
        [-0.3, -0.1],
        [-0.3, 0.1],
        [-0.1, 0.1],
        [-0.1, 0.3],
        [0.1, 0.3],
        [0.1, 0.1],
        [0.3, 0.1],
        [0.3, -0.1],
        [0.1, -0.1],
        [0.1, -0.3],
        [-0.1, -0.3],
        [-0.1, -0.1]
    ]);
    let mut indices = vec![
        0,1,5,
        0,5,4,
        0,4,6,
        0,6,2,
        3,5,1,
        3,7,5,
        3,6,7,
        3,2,6,
    ];
    indices.extend_from_slice(&[
        8, 12, 23,
        8, 23, 22,
        8, 22, 10,
        8, 9, 12,
        9, 13, 12,
        9, 14, 13,
        9, 15, 14,
        9, 16, 15,
        11, 16, 9,
        11, 17, 16,
        11, 18, 17,
        11, 19, 18,
        10, 11, 18,
        10, 19, 18,
        10, 20, 19,
        10, 22, 21,
        10, 21, 20,
    ]);
    indices.extend([
        0,1,6,
        7,0,6,
        10,8,9,
        10,11,8,
        2,4,5,
        2,3,4,
    ].into_iter().map(|x| x+24));

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vert.iter().map(|x| [x[0], x[1], 0.0]).collect::<Vec<_>>());
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![
        border,border,border,border,
        border,border,border,border,
        fill,fill,fill,fill,
        fill,fill,fill,fill,
        fill,fill,fill,fill,
        fill,fill,fill,fill,
        cross,cross,cross,cross,
        cross,cross,cross,cross,
        cross,cross,cross,cross,
    ].into_iter().map(Color::as_linear_rgba_f32).collect::<Vec<_>>());
    mesh.insert_indices(Indices::U32(indices));
    println!("MESSSSSSSH {:?}", mesh);
    mesh
}
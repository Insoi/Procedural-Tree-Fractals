use bevy::log::tracing_subscriber::fmt::init;
use bevy::prelude::*;
use bevy_egui::egui::accesskit::Role::Tree;

pub struct Branch {
    pub transform: Transform,
    pub parent_index: Option<usize>,
    pub depth: u32,
}

pub struct Leaf {
    pub pos: Vec3,
    pub parent_index: usize,
}

fn draw_leaf(
    pos: Vec3,
    parent_index: usize,
    leaves: &mut Vec<Leaf>
) {
    leaves.push(Leaf { pos, parent_index });
}

// recursive function approach towards drawing all the branches
fn draw_branch(
    pos: Vec3,
    direction: Vec3,
    angle: f32,
    current_depth: u32,
    max_depth: u32,
    scale: f32,
    parent_index: Option<usize>,
    branches: &mut Vec<Branch>,
    leaves: &mut Vec<Leaf>
) {
    if current_depth == 0 { return; }

    let end_pos = pos + direction;
    let length = direction.length();

    // 0.0 at root as it gets to 1.0 near tip of current depth/iteration
    let depth_ratio = 1.0 - (current_depth as f32 / current_depth as f32);
    let thickness = current_depth as f32 * 1.5;

    //
    let midpoint = pos + direction * 0.5;
    let rotation = Transform::IDENTITY.looking_to(direction.normalize(), Vec3::Y).rotation;
    let transform = Transform { translation: midpoint, rotation, scale: Vec3::new(thickness * 0.1, length, thickness * 0.1) }

    branches.push(Branch {transform, parent_index, depth: current_depth});
    let current_index = branches.len() - 1;

    // last iteration, drawing a leaf IF leaves exist on this tree type
    if current_depth == 1 {
        draw_leaf(end_pos, current_index, leaves);
        return;
    }

    let up = Vec3::Y;
    let right = if direction.normalize().dot(up).abs() > 0.99 {
        Vec3::X
    } else {
        direction.cross(up).normalize()
    };
    let forward = direction.cross(right).normalize();

    let left_direction = Quat::from_axis_angle(forward, -angle) * direction * scale;
    draw_branch(end_pos, left_direction, angle, current_depth - 1, scale);

    let right_direction = Quat::from_axis_angle(forward, angle) * direction * scale;
    draw_branch(end_pos, right_direction, angle, current_depth - 1, scale);

    let forward_direction = Quat::from_axis_angle(right, angle) * direction * scale;
    draw_branch(end_pos, forward_direction, angle, current_depth - 1, scale);
}

// TODO: add function params rather than hard-coded variables
pub fn generate()  -> (Vec<Branch>, Vec<Leaf>) {
    let start_pos = Vec3::new(0.0, 0.0, 0.0); // root pos
    let initial_direction = Vec3::new(0.0, 3.0, 0.0); // trunk that just directly up
    let angle = 0.45; // angle of branch deviation
    let depth = 6;
    let length_scale = 0.7;

    let mut branches = Vec::new();
    let mut leaves = Vec::new();

    draw_branch(
        start_pos,
        initial_direction,
        angle,
        depth,
        depth,
        length_scale,
        None,
        &mut branches,
        &mut leaves,
    );

    (branches, leaves)
}
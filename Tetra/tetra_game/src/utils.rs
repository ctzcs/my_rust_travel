use egui::color_picker::color_edit_button_srgb;
use egui::emath::align::center_size_in_rect;
use tetra::graphics::Camera;
use tetra::math::{Vec2};

/// camera.project应该就是screen to world,里面的注释写错了
/// 作者说这是因为他之前以为world是相机之外的世界
pub fn screen_to_world(camera: &Camera, screen_pos: Vec2<f32>) -> Vec2<f32> {
    //平移，缩放旋转,再平移回去
    // 1. 将屏幕坐标转换为相对于屏幕中心的坐标
    let centered_pos = Vec2::new(
        screen_pos.x - camera.scale.x / 2.0,
        screen_pos.y - camera.scale.y / 2.0,
    );

    // 2. 应用缩放:抵消相机缩放的影响，比如相机放缩2倍，意思就是真实距离应该是现在的一半
    let scaled_pos = centered_pos / camera.scale;

    // 3. 应用旋转
    let cos_r = camera.rotation.cos();
    let sin_r = camera.rotation.sin();
    let rotated_pos = Vec2::new(
        scaled_pos.x * cos_r - scaled_pos.y * sin_r,
        scaled_pos.x * sin_r + scaled_pos.y * cos_r,
    );
    
    // 4. 应用相机位置偏移
    rotated_pos + camera.position
}

pub fn world_to_camera(camera: &Camera, world_pos:Vec2<f32>) -> Vec2<f32> {
    //相机坐标系
    //平移
    let relative_pos = world_pos - camera.position;

    //旋转放缩
    let cos_r = camera.rotation.cos();
    let sin_r = camera.rotation.sin();
    let rotated_pos = Vec2::new(
        relative_pos.x * cos_r + relative_pos.y * sin_r,
        -relative_pos.x * sin_r + relative_pos.y * cos_r,
    );
    
    Vec2::new(
        rotated_pos.x * camera.scale.x,
        rotated_pos.y * camera.scale.y,
    )
    
}
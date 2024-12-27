use tetra::{Context, input};
use tetra::graphics::Camera;
use tetra::math::{Vec2};

pub fn get_world_mouse_position(ctx: &Context, camera: &Camera)-> Vec2<f32>{
    // 获取鼠标在屏幕上的坐标
    let screen_pos = input::get_mouse_position(ctx);
    // 使用相机的变换矩阵将屏幕坐标转换为世界坐标

    // 正确的坐标转换
    let world_x = screen_pos.x + camera.position.x;
    let world_y = screen_pos.y + camera.position.y;

    if camera.scale != Vec2::one() {
        // 如果有缩放，需要考虑缩放因素
        Vec2::new(
            world_x / camera.scale.x,
            world_y / camera.scale.y
        )
    } else {
        Vec2::new(world_x, world_y)
    }
}
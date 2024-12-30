use tetra::graphics::animation::Animation;
///动画表现
#[derive(Debug)]
pub struct View {
    pub anim:Animation,
}

impl View {
    pub fn new(anim:Animation)-> View {
        View { anim, }
    }
}

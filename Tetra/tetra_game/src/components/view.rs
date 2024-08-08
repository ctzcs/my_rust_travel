use tetra::graphics::animation::Animation;
#[derive(Debug)]
pub struct AnimView{
    pub anim:Animation,
}

impl AnimView {
    
    pub fn new(anim:Animation)->AnimView{
        AnimView{ anim, }
    }
    
}

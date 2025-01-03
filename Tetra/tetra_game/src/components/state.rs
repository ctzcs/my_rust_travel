pub enum EState{
    Running,
    Attacking,
    Avoiding
}


//如果想给怪物实现一个状态机，每种怪物的每个状态有不同的行为，
//state 怪物当前状态，action,逻辑表示怪物当前行为
//动画配置数据应该是一个资源，加载资  源之后知道图片在哪里。
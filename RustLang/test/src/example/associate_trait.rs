trait ITrait{
    type M;
    fn do_sth(&mut self,m:Self::M);
}

struct Model(i32);


struct SomeThing{
    model:Model,
    model1:Model,
}

impl ITrait for SomeThing{
    type M = Model;

    fn do_sth(&mut self, model:Self::M) {
        
    }
}
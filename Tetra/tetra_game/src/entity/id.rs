#[derive(Clone)]
pub struct IdAllocator {
    next_id: u32,
    id_pool:Vec<u32>,
}

impl IdAllocator {
    pub fn new(next_id:u32,id_pool:Vec<u32>)->IdAllocator{
        IdAllocator{
            next_id,
            id_pool,
        }
    }
    pub fn pop_id(&mut self) -> u32{
        if let Some(result)= self.id_pool.pop(){
            result
        } else { 
            self.next_id += 1;
            self.next_id
        }
    }
    
    pub fn push_id(&mut self, id:u32){
        self.id_pool.push(id);
    }
}
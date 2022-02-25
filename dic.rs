use std::boxed::Box;

pub struct Node
{
    data: isize,
    next: Box<Node>
}

pub struct List{
    head: Box<Node>
}

impl Node{
    pub fn insert(&mut self,num:isize){
        
    }
}
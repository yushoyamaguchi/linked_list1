//use std::io;
use std::boxed::Box;


fn type_of<T>(_: T) -> String{
  let a = std::any::type_name::<T>();
  return a.to_string();
}

pub struct Node
{
    is_head: bool,
    data: isize,
    next: Option<Box<Node>>,
}

impl Clone for Node {
    
  fn clone(&self) -> Self {
      Node {
        is_head: self.is_head,
        data: self.data,
        next: self.next.clone()
      }
  }

  fn clone_from(&mut self, source: &Self) {
      *self = source.clone();
  }
}



impl Node{
  pub fn new(data: isize)->Node{
    Node{is_head:false,data:data,next:None}
  }


}

pub fn insert(node:Option<Box<Node>>,data:isize)->Option<Box<Node>>{
  return match node{
    None=>{
      Some(Box::new(Node::new(data)))//変える
    }
    Some(v)=>{
      let mut cur_node=v.clone();
      //println!("passed node of {}",cur_node.data);
      if data<0{
        println!("cannot treat numbers less than zero");
        return None;
      }
      let _next_ref = match &mut cur_node.next {
        Some(next_node)=>{
          if next_node.data<data{
            cur_node.next=insert(cur_node.next,data);
          }
          else if next_node.data==data{
            println!("already exist");
          }
          else if next_node.data>data{
            //let mut previous=&mut cur_node;
            let new_node=Node{
              is_head:false,
              data,
              next:cur_node.next.as_ref().map(|x| x.clone()),
            };
            cur_node.next=Some(Box::new(new_node));
          }
        }
        None=>{
          let new_node=Node{
            is_head:false,
            data,
            next:None,
          };
          cur_node.next=Some(Box::new(new_node));
        }
      };
      Some(cur_node)//変える
    }
  };
}





fn main() {

  let dummy_head=Node{
    is_head:true,
    data:-1,
    next:None
  };

  let mut head=Some(Box::new(dummy_head));
  head=insert(head,4);
  head=insert(head,10);
  head=insert(head,8);
  head=insert(head,18);
  
  //先頭のioのところコメント外す
  /*let mut end_flag=0;
  println!("press q to quit");
  let str_i = String::from("i");
  let str_d = String::from("d");
  let str_q = String::from("q");*/
  /*while end_flag==0{
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("input error");
    let x2 = x.trim().to_string();
    if x2.eq(&str_i)==true{
      println!("insert");
    }
    else if x2.eq(&str_d)==true{
      println!("delete");
    }
    else if x2.eq(&str_q)==true{
      println!("quit");
      end_flag=1;
    }
  }*/
}
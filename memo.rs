pub fn insert(&mut self,data:isize){
    let mut cur_ref = match &mut self.head {
      Some(head_ref) => head_ref,
      None => {
          let node_new = Node::new(data);
          self.head = Some(Box::new(node_new));
          return;
      }
    };  

    if cur_ref.data>data{
      let mut new_head=Node::new(data);
      new_head.next=cur_ref.next.as_ref().map(|x| x.clone());
      self.head = Some(Box::new(new_head));
    }  

    while let Some(ref mut next) = cur_ref.next{
      if next.data>data {
        break;
      }
      cur_ref = next;
    }
    /*let node_new=Node{
      data,
      next: cur_ref.next.as_ref().map(|x| x.clone()),
    };*/
    cur_ref.next = Some(Box::new(Node::new(data)));

  }
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct VersionNode<T>{
    // 下一个版本节点 可能有多个下一个版本 所以使用Vec存储
    next_node:Option<Vec<Rc<RefCell<VersionNode<T>>>>>,
    // 上一个版本节点
    last_node:Option<Rc<RefCell<VersionNode<T>>>>,
    // commit_info:Commit;
    data:T
}

impl<T> VersionNode<T>{
    pub fn new(data:T)->Self{
        VersionNode{
            data,
            last_node:None,
            next_node:None
        }
    }
}

#[derive(Debug)]
pub struct VersionNodeList<T>{
    head:Option<Rc<RefCell<VersionNode<T>>>>,
    tail:Option<Rc<RefCell<VersionNode<T>>>>,
}

impl<T> VersionNodeList<T>  {
    pub fn new()->Self{
        Self{
            head:None,
            tail:None
        }
    }
    pub fn append(&mut self, data:Rc<RefCell<VersionNode<T>>>){
        let node=Some(data);
        match self.tail.take(){
            Some(t)=>{
                    self.tail=node;
            },
            None=>{
                    self.head=node.clone();
                    self.tail=node;
            }
        }
    }
}
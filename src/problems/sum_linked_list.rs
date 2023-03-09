// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    
    let mut over_flow = 0;
    let mut nodes= vec![];

    loop {
        let l1_node_val;
        let l2_node_val;

        if let None = l1 {
            if let None = l2 {
                break;
            }
        }

        if let Some(node_val) = &l1{
            l1_node_val = node_val.val;
        } else {
            l1_node_val = 0;
        }

        if let Some(node_val) = &l2{
            l2_node_val = node_val.val;
        } else {
            l2_node_val = 0;
        }

        let mut new_val = l1_node_val + l2_node_val + over_flow;
        over_flow = 0;

        if new_val >= 10 {
            let str = new_val.to_string();
            let mut chars = str.chars();
            over_flow = (chars.next().unwrap() as u8 - b'0') as i32;
            new_val = (chars.next().unwrap() as u8 - b'0') as i32;
        }

        let new_node = Box::new(ListNode::new(new_val));
        nodes.push(new_node);

        // change l1
        if let Some(node) = l1 {
            l1 = node.next;
        } else {
            l1 = None;
        }

        if let Some(node) = l2 {
            l2 = node.next;
        } else {
            l2 = None;
        }
    }

    if over_flow != 0{
        let last_node = Box::new(ListNode::new(over_flow));
        nodes.push(last_node);
    }

    for i in 0..nodes.len(){
        let current_idx =  nodes.len()-i-1;
        if current_idx == 0{
            break;
        }

        nodes[current_idx-1].next = Some(nodes[current_idx].clone());
    }

    Some(nodes.remove(0))
}

#[test]
fn test_code(){
    
}
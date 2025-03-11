use anyhow::Result;
// Input a: 1->2->3->4
// Input b: 2->3->4
// Output: 1->4->6->8
#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() -> Result<()> {
    let a = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    };

    let b = ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    };

    let c = add_two_numbers(Some(Box::new(a)), Some(Box::new(b)));
    println!("{:?}", c);

    Ok(())
}

fn add_two_numbers(
    a: Option<Box<ListNode>>,
    b: Option<Box<ListNode>>,
) -> Result<Option<Box<ListNode>>> {
    let mut a = a;
    let mut b = b;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut head;
    let mut carry = 0;

    while a.is_some() || b.is_some() {
        let sum =
            carry + a.as_ref().map_or(0, |node| node.val) + b.as_ref().map_or(0, |node| node.val);
        carry = sum / 10;
        let node = ListNode::new(sum % 10);
        tail.as_mut().unwrap().next = Some(Box::new(node));
        tail = &mut tail.as_mut().unwrap().next;
        a = a.and_then(|node| node.next);
        b = b.and_then(|node| node.next);
    }

    if carry > 0 {
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
    }

    Ok(head.unwrap().next)
}

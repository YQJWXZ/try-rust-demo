use anyhow::Result;
// Input a: 1->2->3->4
// Input b: 2->3->4
// Output: 1->4->6->8
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

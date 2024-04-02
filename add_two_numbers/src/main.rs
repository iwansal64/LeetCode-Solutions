// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1: Option<Box<ListNode>> = l1;
    let mut l2: Option<Box<ListNode>> = l2;
    let mut res: Vec<Option<i32>> = Vec::new();

    let mut remaining = 0;
    while l1 != Option::None || l2 != Option::None {

        let mut current_num: i32 = 0;
        
        match l1 {
            Some(l1_num) => {
                current_num += l1_num.val;
                l1 = l1_num.next;
            },
            None => l1 = None
        }
        
        match l2 {
            Some(l2_num) => {
                current_num += l2_num.val;
                l2 = l2_num.next;
            },
            None => l2 = None
        }

        current_num += remaining;
        
        if current_num >= 10 {
            current_num -= 10;
            if l1 != Option::None || l2 != Option::None {
                remaining = 1;
            }
            else {
                remaining = 0;
                l1 = Some(Box::new(ListNode {
                    val: 1,
                    next: None
                }))
            }
        }
        else {
            remaining = 0;
        }


        res.push(Some(current_num));
    }

    let mut return_value: Option<Box<ListNode>> = None;

    for num in res.iter().rev() {
        match num {
            Some(val) => {
                return_value = Some(Box::new(ListNode {
                    val: val.clone(),
                    next: return_value 
                }))
            },
            None => ()
        }
    }
    
    return_value
}

fn main() {
    let mut result = add_two_numbers( Some(Box::new(ListNode {
            // val: 5,
            // next: Some(Box::new(ListNode {
            //     val: 6,
            //     next: Some(Box::new(ListNode {
            //         val: 4,
            //         next: None
            //     }))
            // }))
            val: 9,
            next: None
        })), Some(Box::new(ListNode {
            // val: 1,
            // next: Some(Box::new(ListNode {
            //     val: 9,
            //     next: Some(Box::new(ListNode {
            //         val: 9,
            //         next: Some(Box::new(ListNode {
            //             val: 9,
            //             next: Some(Box::new(ListNode {
            //                 val: 9,
            //                 next: Some(Box::new(ListNode {
            //                     val: 9,
            //                     next: Some(Box::new(ListNode {
            //                         val: 9,
            //                         next: None
            //                     }))
            //                 }))
            //             }))
            //         }))
            //     }))
            // }))
            val: 9,
            next: None
        }))
    );

    while let Some(res) = result {
        print!("{}", res.val);
        result = res.next;
    }
}
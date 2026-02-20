use crate::linked_list::LinkedList;

pub mod linked_list;
pub mod node;

fn main() {
    let mut linked_list = LinkedList::<u32>::default();

    dbg!(&linked_list);

    linked_list.insert(1);

    dbg!(&linked_list);

    linked_list.insert(2);

    dbg!(&linked_list);

    linked_list.insert(3);

    dbg!(&linked_list);
}

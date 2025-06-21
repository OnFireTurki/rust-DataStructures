mod linked_list;

use linked_list::dll::DList;

fn main() {
    let mut t = DList::new();
    t.push_back(10);
    t.push_front(12);
    t.push_front(12);
    t.push_back(12);
    t.traverse();
    for i in 1..3{
        t.pop_back();
        t.pop_back();
        t.pop_back();
        t.traverse();
    }
}

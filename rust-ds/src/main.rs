mod linked_list;

use linked_list::sll::List;

fn main() {
    let mut t = List::new();
    t.push_back(10);
    t.push_front(12);
    t.push_front(12);
    t.push_back(12);
    println!("{}", t.len());
    t.traverse();
}

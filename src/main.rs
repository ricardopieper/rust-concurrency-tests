mod linked_set;
use linked_set::LinkedSet;
use linked_set::Set;

fn main() {
    let mut set = LinkedSet::<i32>::new();
    set.add(1);
    println!("Hello, world!");
}

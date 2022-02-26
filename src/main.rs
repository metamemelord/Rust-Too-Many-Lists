mod first;

fn main() {
    let mut l: first::LinkedList<i32> = first::LinkedList::new();
    l.insert(10);
    l.insert(20);
    println!("Size of ll is {}", l.size());
    l.print();
}

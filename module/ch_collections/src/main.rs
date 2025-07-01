use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    println!("Hello, world!");
}

#[test]
fn fn_vec() {
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    vec.push(6);
    println!("Vector: {:?}", vec);
}

#[test]
fn fn_vec_deque() {
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_front(1);
    deque.push_back(2);
    deque.push_front(3);
    println!("Deque: {:?}", deque);
    if let Some(front) = deque.pop_front() {
        println!("Popped from front: {}", front);
    }
    println!("Deque: {:?}", deque);
    if let Some(back) = deque.pop_back() {
        println!("Popped from back: {}", back);
    }
    println!("Deque: {:?}", deque);
}

#[test]
fn fn_linked_list() {
    let mut linked_list: LinkedList<i32> = LinkedList::new();
    linked_list.push_back(1);
    linked_list.push_back(2);
    linked_list.push_front(0);
    println!("Linked List: {:?}", linked_list);
    if let Some(front) = linked_list.pop_front() {
        println!("Popped from front: {}", front);
    }
    println!("Linked List: {:?}", linked_list);
    if let Some(back) = linked_list.pop_back() {
        println!("Popped from back: {}", back);
    }
    println!("Linked List: {:?}", linked_list);
}

#[test]
fn fn_hash_map() {
    let mut hash_map: HashMap<&str, i32> = HashMap::new();
    hash_map.insert("Alice", 30);
    hash_map.insert("Bob", 25);
    println!("Hash Map: {:?}", hash_map);
    if let Some(age) = hash_map.get("Alice") {
        println!("Alice's age: {}", age);
    }
    if let Some(age) = hash_map.remove("Bob") {
        println!("Removed Bob's age: {}", age);
    }
}

#[test]
fn btree_map() {
    let mut btree_map: BTreeMap<&str, i32> = BTreeMap::new();
    btree_map.insert("Alice", 30);
    btree_map.insert("Bob", 25);
    btree_map.insert("Charlie", 35);
    btree_map.insert("Dave", 20);

    println!("BTree Map: {:?}", btree_map);
    if let Some(age) = btree_map.get("Alice") {
        println!("Alice's age: {}", age);
    }
    if let Some(age) = btree_map.remove("Bob") {
        println!("Removed Bob's age: {}", age);
    }

    for (key, value) in btree_map.range("Alice"..="Dave") {
        println!("Key: {}, Value: {}", key, value);
    }
}
#[test]
fn hash_set() {
    let mut set = HashSet::new();
    set.insert("Alice");
    set.insert("Bob");
    set.insert("Charlie");
    println!("Hash Set: {:?}", set);
    if set.contains("Alice") {
        println!("Set contains Alice");
    }
    if set.remove("Bob") {
        println!("Removed Bob from the set");
    }
    println!("Hash Set after removal: {:?}", set);
}

#[test]
fn btree_set() {
    let mut btree_set = BTreeSet::new();
    btree_set.insert("Alice");
    btree_set.insert("Bob");
    btree_set.insert("Charlie");
    println!("BTree Set: {:?}", btree_set);
    if btree_set.contains("Alice") {
        println!("BTree Set contains Alice");
    }
    if btree_set.remove("Bob") {
        println!("Removed Bob from the BTree Set");
    }
    println!("BTree Set after removal: {:?}", btree_set);
}
#[test]
fn fn_binary_heap() {
    let mut heap:BinaryHeap<i32> = BinaryHeap::new();
    heap.push(5);
    heap.push(1);
    heap.push(3);
    println!("Binary Heap: {:?}", heap);
    if let Some(max) = heap.pop() {
        println!("Popped max from heap: {}", max);
    }
    println!("Binary Heap after pop: {:?}", heap);
}

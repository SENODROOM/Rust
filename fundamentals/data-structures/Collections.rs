// 12_collections.rs

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, BinaryHeap};

fn main() {
    // =========================================
    // VEC<T>
    // =========================================
    println!("=== Vec<T> ===");
    let mut v: Vec<i32> = Vec::new();
    for i in 1..=5 { v.push(i * 10); }
    println!("Vec: {:?}", v);

    v.insert(2, 99);
    println!("After insert(2, 99): {:?}", v);

    v.remove(2);
    println!("After remove(2): {:?}", v);

    println!("len: {}, contains(30): {}", v.len(), v.contains(&30));
    println!("get(1): {:?}, [1]: {}", v.get(1), v[1]);

    // Sorting
    let mut nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    nums.sort();
    println!("Sorted: {:?}", nums);
    nums.dedup();
    println!("Deduped: {:?}", nums);
    nums.sort_by(|a, b| b.cmp(a));
    println!("Reverse sorted: {:?}", nums);

    // retain
    let mut evens = vec![1, 2, 3, 4, 5, 6, 7, 8];
    evens.retain(|&x| x % 2 == 0);
    println!("Retain evens: {:?}", evens);

    // extend and drain
    let mut base = vec![1, 2, 3];
    base.extend([4, 5, 6]);
    println!("Extended: {:?}", base);

    let drained: Vec<i32> = base.drain(1..4).collect();
    println!("Drained 1..4: {:?}, remaining: {:?}", drained, base);

    // Slices and windows
    let data = vec![1, 2, 3, 4, 5, 6];
    let windows: Vec<&[i32]> = data.windows(3).collect();
    println!("Windows(3): {:?}", windows);
    let chunks: Vec<&[i32]> = data.chunks(2).collect();
    println!("Chunks(2): {:?}", chunks);

    // split_at
    let (left, right) = data.split_at(3);
    println!("split_at(3): {:?} | {:?}", left, right);

    // =========================================
    // HASHMAP<K, V>
    // =========================================
    println!("\n=== HashMap<K, V> ===");
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Carol"), 92);

    // Access
    println!("Alice: {:?}", scores.get("Alice"));
    println!("Dave: {:?}", scores.get("Dave"));
    println!("Contains Bob: {}", scores.contains_key("Bob"));

    // Iterate (order not guaranteed)
    let mut entries: Vec<(&String, &i32)> = scores.iter().collect();
    entries.sort_by_key(|&(k, _)| k);
    for (name, score) in &entries {
        println!("  {} -> {}", name, score);
    }

    // Update
    scores.insert(String::from("Alice"), 110); // overwrites
    *scores.get_mut("Bob").unwrap() += 5;      // modify in place
    println!("Alice updated: {:?}", scores.get("Alice"));

    // Entry API
    scores.entry(String::from("Dave")).or_insert(75);   // insert if missing
    scores.entry(String::from("Alice")).or_insert(999); // won't change (exists)
    println!("Dave: {:?}", scores.get("Dave"));
    println!("Alice unchanged: {:?}", scores.get("Alice"));

    // Word frequency counter
    let text = "hello world hello rust world hello";
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }
    let mut wc_sorted: Vec<(&&str, &u32)> = word_count.iter().collect();
    wc_sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("Word counts: {:?}", wc_sorted);

    // Remove
    scores.remove("Carol");
    println!("After remove Carol: {} entries", scores.len());

    // Collect from iterator
    let pairs = vec![("x", 1), ("y", 2), ("z", 3)];
    let map: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("Collected map: {:?}", map);

    // =========================================
    // HASHSET<T>
    // =========================================
    println!("\n=== HashSet<T> ===");
    let mut set: HashSet<i32> = HashSet::new();
    for x in [1, 2, 3, 4, 5, 3, 2, 1] { set.insert(x); }
    println!("Set: {:?}", set);
    println!("Contains 3: {}, contains 6: {}", set.contains(&3), set.contains(&6));

    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [3, 4, 5, 6, 7].iter().cloned().collect();

    let mut inter: Vec<i32> = set_a.intersection(&set_b).cloned().collect();
    inter.sort();
    let mut union: Vec<i32> = set_a.union(&set_b).cloned().collect();
    union.sort();
    let mut diff: Vec<i32> = set_a.difference(&set_b).cloned().collect();
    diff.sort();
    let mut sym: Vec<i32> = set_a.symmetric_difference(&set_b).cloned().collect();
    sym.sort();

    println!("A: {:?}", { let mut v: Vec<i32> = set_a.iter().cloned().collect(); v.sort(); v });
    println!("B: {:?}", { let mut v: Vec<i32> = set_b.iter().cloned().collect(); v.sort(); v });
    println!("A ∩ B: {:?}", inter);
    println!("A ∪ B: {:?}", union);
    println!("A - B: {:?}", diff);
    println!("A △ B: {:?}", sym);
    println!("A ⊆ A: {}", set_a.is_subset(&set_a));
    println!("A ⊆ B: {}", set_a.is_subset(&set_b));

    // =========================================
    // BTREEMAP<K, V> — SORTED MAP
    // =========================================
    println!("\n=== BTreeMap<K, V> ===");
    let mut btree: BTreeMap<&str, i32> = BTreeMap::new();
    btree.insert("banana", 3);
    btree.insert("apple", 5);
    btree.insert("cherry", 1);
    btree.insert("date", 2);

    // Iterates in sorted key order
    println!("BTreeMap (sorted by key):");
    for (k, v) in &btree { println!("  {} -> {}", k, v); }

    // Range queries
    println!("Range apple..=cherry:");
    for (k, v) in btree.range("apple"..="cherry") {
        println!("  {} -> {}", k, v);
    }

    // First and last
    println!("First: {:?}", btree.iter().next());
    println!("Last: {:?}", btree.iter().next_back());

    // BTreeSet — sorted unique values
    let mut bset: BTreeSet<i32> = BTreeSet::new();
    for x in [5, 3, 8, 1, 4, 2, 7, 6] { bset.insert(x); }
    println!("BTreeSet (sorted): {:?}", bset.iter().collect::<Vec<_>>());

    // =========================================
    // VECDEQUE<T> — DOUBLE-ENDED QUEUE
    // =========================================
    println!("\n=== VecDeque<T> ===");
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    deque.push_front(0);
    deque.push_front(-1);
    println!("Deque: {:?}", deque);

    println!("pop_front: {:?}", deque.pop_front());
    println!("pop_back: {:?}", deque.pop_back());
    println!("Remaining: {:?}", deque);

    // Use as queue (FIFO)
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(String::from("task1"));
    queue.push_back(String::from("task2"));
    queue.push_back(String::from("task3"));
    while let Some(task) = queue.pop_front() {
        println!("Processing: {}", task);
    }

    // =========================================
    // BINARYHEAP<T> — PRIORITY QUEUE
    // =========================================
    println!("\n=== BinaryHeap<T> ===");
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    for x in [3, 1, 4, 1, 5, 9, 2, 6] { heap.push(x); }
    
    println!("Peek (max): {:?}", heap.peek());
    print!("Pop order (max-first): ");
    while let Some(val) = heap.pop() {
        print!("{} ", val);
    }
    println!();

    // Min-heap using Reverse
    use std::cmp::Reverse;
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for x in [3, 1, 4, 1, 5, 9] { min_heap.push(Reverse(x)); }
    print!("Min-heap order: ");
    while let Some(Reverse(val)) = min_heap.pop() {
        print!("{} ", val);
    }
    println!();

    // =========================================
    // COLLECTION CONVERSIONS
    // =========================================
    println!("\n=== Conversions ===");
    let v: Vec<i32> = (1..=5).collect();
    let set: HashSet<i32> = v.iter().cloned().collect();
    let sorted: BTreeSet<i32> = v.iter().cloned().collect();
    let heap: BinaryHeap<i32> = v.iter().cloned().collect();
    println!("Vec: {:?}", v);
    println!("Set: {:?}", set);
    println!("BTreeSet: {:?}", sorted);
    println!("Heap max: {:?}", heap.peek());
}

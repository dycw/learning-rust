fn main() {
    let arr = [6, 8, 2, 8, 4, 9, 6, 1, 8, 0];
    let mut v = Vec::<_>::new();
    let mut hs = std::collections::HashSet::<_>::new();
    let mut bs = std::collections::BTreeSet::<_>::new();
    for i in arr.iter() {
        v.push(i);
        hs.insert(i);
        bs.insert(i);
    }
    print!("Vec:");
    for i in v.iter() {
        print!(" {}", i);
    }
    println!(". {:?}", v);
    print!("HashSet :");
    for i in hs.iter() {
        print!(" {}", i);
    }
    println!(". {:?}", hs);
    print!("BTreeSet:");
    for i in bs.iter() {
        print!(" {}", i);
    }
    println!(". {:?}", bs);
}

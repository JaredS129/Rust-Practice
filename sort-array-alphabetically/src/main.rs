enum SortOrder {
    Ascending,
    Descending,
}

fn sort_array(arr: &mut Vec<&str>, order: SortOrder) {
    match order {
        SortOrder::Ascending => arr.sort(),
        SortOrder::Descending => arr.sort_by(|a, b| b.cmp(a)),
    }
}

fn main() {
    let mut arr: Vec<&str> = vec!["z", "a", "b", "c", "d", "e", "f", "g", "h", "i"];
    println!("Before sorting: {:?}", arr);
    sort_array(&mut arr, SortOrder::Ascending);
    println!("After ascending sort: {:?}", arr);
    sort_array(&mut arr, SortOrder::Descending);
    println!("After descending sort: {:?}", arr);
}

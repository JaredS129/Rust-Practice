enum SortOrder {
    Ascending,
    Descending,
}

fn sort_array(arr: &[char; 10], order: SortOrder) -> [char; 10] {
    match order {
        SortOrder::Ascending => {
            let mut arr: [char; 10] = *arr;
            arr.sort();
            return arr;
        }
        SortOrder::Descending => {
            let mut arr: [char; 10] = *arr;
            arr.sort_by(|a, b| b.cmp(a));
            return arr;
        }
    }
}

fn main() {
    let original_array: [char; 10] = ['z', 'a', 'c', 'b', 'd', 'e', 'f', 'g', 'h', 'i'];
    println!("Before sorting: {:?}", &original_array);
    let ascending_array: [char; 10] = sort_array(&original_array, SortOrder::Ascending);
    println!("After ascending sort: {:?}", ascending_array);
    let descending_array: [char; 10] = sort_array(&original_array, SortOrder::Descending);
    println!("After descending sort: {:?}", descending_array);
}

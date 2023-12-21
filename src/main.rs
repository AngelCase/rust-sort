mod merge_sort;

fn main() {
    let mut arr = vec![5, 4, 3, 2, 1];
    merge_sort::sort(&mut arr);
    println!("{:#?}", arr);
}

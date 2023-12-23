mod merge_sort;
mod quick_sort;

fn main() {
    println!("------------");
    println!(" merge sort");
    println!("------------");
    let mut arr = vec![5, 4, 3, 2, 1];
    merge_sort::sort(&mut arr);
    println!("{:#?}", arr);

    println!("------------");
    println!(" quick sort");
    println!("------------");
    let mut arr = vec![5, 2, 4, 4, 1];
    quick_sort::sort(&mut arr);
    println!("{:#?}", arr);
}

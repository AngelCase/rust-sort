pub fn sort(arr: &mut Vec<i32>) {
    if arr.len() == 0 {
        return;
    }
    helper(arr, 0, (arr.len() - 1) as i32);
}

fn helper(arr: &mut Vec<i32>, start: i32, last: i32) {
    if last <= start {
        return;
    }

    let pivot = partition(arr, start, last);

    helper(arr, start, pivot - 1);
    helper(arr, pivot + 1, last);
}

fn partition(arr: &mut Vec<i32>, start: i32, last: i32) -> i32 {
    let pivot = (start + last) / 2;
    let pivot = arr[pivot as usize];

    let mut i = start - 1;
    let mut j = last + 1;

    // pivotの左側はpivotより小さく、pivotの右側はpivotより大きくなるように移動させる
    loop {
        loop {
            i += 1;
            if pivot <= arr[i as usize] {
                break;
            }
        }

        loop {
            j -= 1;
            if arr[j as usize] <= pivot {
                break;
            }
        }

        if j <= i {
            return i;
        }

        arr.swap(i as usize, j as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        let mut arr = vec![2, 4, 3, 5, 1];

        sort(&mut arr);

        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];

        sort(&mut arr);

        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn reversed() {
        let mut arr = vec![5, 4, 3, 2, 1];

        sort(&mut arr);

        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn contains_duplicate() {
        let mut arr = vec![5, 2, 4, 4, 1];

        sort(&mut arr);

        assert_eq!(arr, vec![1, 2, 4, 4, 5]);
    }

    #[test]
    fn empty() {
        let mut arr = vec![];

        sort(&mut arr);

        assert_eq!(arr, vec![]);
    }

    #[test]
    fn only_1_element() {
        let mut arr = vec![3];

        sort(&mut arr);

        assert_eq!(arr, vec![3]);
    }
}

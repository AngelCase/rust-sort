pub fn sort(arr: &mut Vec<i32>) {
    helper(arr, 0, arr.len());
}

// rightは末尾の要素の次の要素番号を指している点に注意
fn helper(arr: &mut Vec<i32>, left: usize, right: usize) {
    // 要素数が1の時は何もしない
    if left == right || left == right - 1 {
        return;
    }

    let mid = (left + right) / 2;

    // 2分割
    helper(arr, left, mid);
    helper(arr, mid, right);

    // ソートしながらマージ
    merge(arr, left, mid, right);
}

// rightは末尾の要素の次の要素番号を指している点に注意
fn merge(arr: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
    let mut v = Vec::new();

    // 左半分と右半分から、昇順にデータを詰め込む
    let mut i = left;
    let mut j = mid;
    while i < mid && j < right {
        if arr[i] < arr[j] {
            v.push(arr[i]);
            i += 1;
        } else {
            v.push(arr[j]);
            j += 1;
        }
    }

    // 左半分が先に終わったら右半分を詰め込む
    while i == mid && j < right {
        v.push(arr[j]);
        j += 1;
    }
    // 右半分が先に終わったら左半分を詰め込む
    while j == right && i < mid {
        v.push(arr[i]);
        i += 1;
    }

    // もとの配列に代入
    for i in 0..v.len() {
        arr[left + i] = v[i];
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

pub fn bubble_sort<T: PartialOrd>(array: &mut [T]) {

    let size = array.len();
    // 单个元素集合或者空集合 直接返回 不需要排序
    if size <= 1 {
        return;
    }

    // 对所有元素进行重复，除了最后一个
    for i in 0..(size-1) {
        // 记录是否发生过交换
        let mut swapped = false;

        for j in 1..(size - i) {
            // 比较相邻的元素 如果第一个比第二个大，就交换他们两个。
            if array[j - 1]  > array[j] {
                // 交换两个元素
                array.swap(j - 1, j);
                swapped = true;
            }
        }

        // 如果未发生交换，说明已经排好序，跳出后续的冒泡
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        bubble_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        bubble_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        );
    }
}

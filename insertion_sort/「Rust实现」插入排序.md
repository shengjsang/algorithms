# 「Rust实现」插入排序

## 概念
**插入排序(InsertionSort)**，一般也被称为直接插入排序。

对于少量元素的排序，它是一个有效的算法。插入排序是一种最简单的排序方法，它的基本思想是将一个记录插入到已经排好序的有序表中，从而一个新的、记录数增 1 的有序表。

在其实现过程使用双层循环，外层循环对除了第一个元素之外的所有元素，内层循环对当前元素前面有序表进行待插入位置查找，并进行移动。

## 复杂度
### 时间复杂度
+ 平均时间复杂度
```math
O(n^2)
```
+ 最好情况
```math
O(n)
```
+ 最坏情况
```math
O(n^2)
```

### 空间复杂度
```math
O(1)
```

### 稳定性
+ 稳定


## 代码

```rust 
pub fn insertion_sort<T: PartialOrd>(array: &mut [T]) {
    // 从第二个元素开始进行插入
    for i in 1..array.len() {
        // 当前元素插入索引
        let mut j = i;
        // 比较大小 如果前一个数比后一个数大 直到它到第一个或者前面的值比它小
        while j > 0 && array[j - 1] > array[j] {
            // 交换位置
            array.swap(j-1, j);
            // 交换后如果前面还有元素再进行重复比较
            j -= 1;
        }
    }
}

// 另一种方法 直接调用
// 这里需要 T: Ord 是因为 binary_search() 方法的限制
// 二分查找查到了返回Ok(元素位置) 找不到返回Err(应该插入位置) 
// 我们直接把应该插入位置提取出来即可
pub fn insertion_sort_binary_search<T: Ord>(arr: &mut[T]) {
    // 从第二个元素开始排序
    for i in 1..arr.len() {
        // 利用二分查找获取 arr[i] 应该插入的位置
        let pos = arr[..i].binary_search(&arr[i]).unwrap_or_else(|pos| pos);
        let mut j = i;
        while j > pos {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod insertion_sort {
        use super::*;

        #[test]
        fn test_empty_vec() {
            let mut empty_vec: Vec<String> = vec![];
            insertion_sort(&mut empty_vec);
            assert_eq!(empty_vec, Vec::<String>::new());
        }

        #[test]
        fn test_number_vec() {
            let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
            insertion_sort(&mut vec);
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
            insertion_sort(&mut vec);
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

    mod insertion_sort_binary_search {
        use super::*;

        #[test]
        fn test_empty_vec() {
            let mut empty_vec: Vec<String> = vec![];
            insertion_sort_binary_search(&mut empty_vec);
            assert_eq!(empty_vec, Vec::<String>::new());
        }

        #[test]
        fn test_number_vec() {
            let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
            insertion_sort_binary_search(&mut vec);
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
            insertion_sort_binary_search(&mut vec);
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
}
```

## 参考

[1] 菜鸟教程 [插入排序](https://www.runoob.com/w3cnote/insertion-sort.html)
[2] Rust算法教程 [插入排序](https://algos.rs/sorting/insertion-sort.html)
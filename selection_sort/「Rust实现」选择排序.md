# 「Rust实现」选择排序

## 概念
从未排序的序列中找到最小（大）的元素放到排序序列的末尾位置，重复直至所有的元素已经排序完成

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
+ 不稳定

## 动图演示
![](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2022/selectionSort.gif)

## 代码
```rust
pub fn selection_sort<T: PartialOrd>(array: &mut [T]) {
    let size = array.len();
    if size <= 1 {
        return;
    }

    for i in 0..(size - 1) {
        let mut min_index = i;
        for j in (i + 1)..size {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            array.swap(i, min_index);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        selection_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        selection_sort(&mut vec);
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
        selection_sort(&mut vec);
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
```

## 参考

[1] 菜鸟教程 [选择排序](https://www.runoob.com/w3cnote/selection-sort.html)
[2] Rust算法教程 [选择排序](https://algos.rs/sorting/selection-sort.html)

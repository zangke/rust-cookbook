//! Filename: ch01_02.rs
//! Description: Vector 排序
//! Date: 2024/06/05 08:36:09
//!

/// 整数 Vector 排序
///
///  vec::sort 对一个整数 Vector 进行排序
/// 使用 vec::sort_unstable 速度更快一些，但不保持相等元素的顺序
fn sort_int() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
/// 浮点数 Vector 排序
///
/// 使用 vec::sort_by 和 PartialOrd::partial_cmp 对其进行排序
fn sort_float() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}

/// 为了使 Person 可排序，你需要四个 traits：Eq、PartialEq、Ord，以及 PartialOrd。
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

/// 结构体 Vector 排序
///
/// 也可以使用 vec:sort_by 方法自定义比较函数，仅按照年龄排序
fn sort_struct() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // 根据获得的自然顺序（name 和 age）对 people 进行排序
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]
    );

    // 根据 age 值对 people 进行排序
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]
    );
}
fn main() {
    sort_int();
    sort_float();
    sort_struct();
}

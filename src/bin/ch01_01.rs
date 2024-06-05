//! Filename: ch0101.rs
//! Description: 生成随机值
//! Date: 2024/06/04 21:42:05

use rand::distributions::{Alphanumeric, Distribution, Standard};
use rand::{thread_rng, Rng};
use rand_distr::{Normal, NormalError, Uniform};

/// 生成随机数
fn gen_rand_example() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>()); // [0-1)
}
/// 生成范围内随机数.
///
/// 使用 Rng::gen_range，在半开放的 [0, 10) 范围内（不包括 10）生成一个随机值。
fn gen_range_example() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}

/// 使用 Uniform 模块可以得到均匀分布的值。在相同范围内重复生成数字时，性能可能会更好
fn uniform_example() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

/// 生成给定分布随机数
///
/// 默认情况下，随机数在 rand crate 中是均匀分布
/// rand_distr crate 提供其它的分布类型
fn gen_rand_distr() -> Result<(), NormalError> {
    let mut rng = thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
/// 为 Standard 实现 Distribution trait，以允许随机生成。
impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

/// 生成自定义类型随机值.
///
/// 随机生成一个元组 (i32, bool, f64) 和用户定义类型为 Point 的变量。
fn gen_custom_rand() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

/// 随机生成一个给定长度的 ASCII 字符串，范围为 A-Z，a-z，0-9，使用字母数字样本
fn create_rand_string() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}

/// 随机生成一个给定长度的 ASCII 字符串.
fn create_rand_password() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
fn main() {
    gen_rand_example();
    gen_range_example();
    uniform_example();
    gen_rand_distr();
    gen_custom_rand;
    create_rand_string();
    create_rand_password();
}

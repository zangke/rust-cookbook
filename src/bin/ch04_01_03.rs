//! Filename: ch04_01_03.rs
//! Description: 在两个线程间传递数据
//! Date: 2024/06/12 08:57:46

use std::{thread, time};
use crossbeam_channel::unbounded;

fn main() {
    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    //在两个线程之间，使用 crossbeam_channel::unbounded 信道交换数据，这意味着可存储消息的数量没有限制。生产者线程在消息之间休眠半秒。
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    }).unwrap();
    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }
}

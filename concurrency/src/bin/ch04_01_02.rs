//! Filename: ch04_01_02.rs
//! Description: 显式线程-- 创建并发的数据管道
//! Date: 2024/06/12 08:53:15
//!

use crossbeam_channel::bounded;
use std::thread;
use std::time::Duration;

fn main() {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // 生产者线程
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }
            // 关闭信道 —— 这是退出的必要条件
            // for 巡海在工作线程中
            drop(snd1);
        });

        // 由 2 个县城并行处理
        for _ in 0..n_workers {
            // 从数据源发送数据到接收器，接收器接收数据
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            // 在不同的线程中衍生工人
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                // 接收数据，直到信道关闭前
                for msg in recvr.iter() {
                    println!("Worker {:?} received {}.", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        // 关闭信道，否则接收器不会关闭
        // 退出 for 循坏
        drop(snd2);

        // 接收器
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap();
}

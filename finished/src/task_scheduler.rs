//use std::cmp::Reverse;
//use std::collections::BinaryHeap;
//use std::iter::FromIterator;
//
///// 同种类任务被整合为一个 Task 对象。
///// 在任务队列中，`last_executed_time` 越小（越早被执行过）、`count` 越大（剩余数量越多）的任务越优先被执行。
///// 每当该类型的任务被执行一次，`last_executed_time` 就会被更新为当前的时间，`count` 会被减一，
///// 当 `count == 0` 时，所有该类型的任务都已经执行完成，该对象会被移除出任务队列。
//#[derive(Ord, PartialOrd, Eq, PartialEq)]
//struct Task {
//    last_executed_time: Reverse<i32>,
//    count: i32,
//}
//
//pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
//    let mut queue = init_task_queue(tasks);
//    let mut time = 0i32;
//
//    while !queue.is_empty() {
//        // 按时间片处理任务队列
//        time += 1;
//
//        // 1. 读取任务队列的状态，确定要执行的动作
//        //   1.1. 如果队首的任务过了冷却期，应当被执行
//        //   1.2. 如果应当执行队首的任务，且只剩下最后一个任务，应当在执行后被移出队列
//        let mut should_execute = false;
//        let mut should_pop = false;
//        let task = queue.peek().unwrap();
//        if time.saturating_sub(task.last_executed_time.0) > n {
//            should_execute = true;
//            if task.count <= 1 {
//                should_pop = true;
//            }
//        }
//
//        // 2. 执行动作
//        if should_pop {
//            queue.pop();
//        } else if should_execute {
//            let mut task = queue.peek_mut().unwrap();
//            task.last_executed_time.0 = time;
//            task.count -= 1;
//        }
//    }
//    time
//}
//
///// 用二叉堆实现的优先队列
//fn init_task_queue(tasks: Vec<char>) -> BinaryHeap<Task> {
//    let mut task_counts = [0i32; 26];
//    for task in tasks {
//        task_counts[((task as u8) - b'A') as usize] += 1;
//    }
//    BinaryHeap::from_iter(
//        task_counts
//            .into_iter()
//            .filter(|&&count| count > 0)
//            .map(|&count| Task {
//                // 确保没有被执行过的任务会优先执行
//                last_executed_time: Reverse(std::i32::MIN),
//                count,
//            }),
//    )
//}

use std::cmp::Reverse;
use std::iter::FromIterator;

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut tasks: Vec<Reverse<i32>> = {
        let mut task_counts = [0i32; 26];
        for task in tasks {
            task_counts[((task as u8) - b'A') as usize] += 1;
        }
        Vec::from_iter(
            task_counts
                .iter()
                .filter(|&&count| count > 0)
                .map(|&count| Reverse(count)),
        )
    };
    tasks.sort();

    let mut time = 0;
    let mut last_len = tasks.len();
    loop {
        time += n + 1;
        tasks
            .iter_mut()
            .take((n + 1) as usize)
            .for_each(|task| task.0 -= 1);
        tasks.sort();

        while let Some(Reverse(count)) = tasks.last() {
            if *count <= 0 {
                tasks.pop();
            } else {
                break;
            }
        }

        if tasks.is_empty() {
            break;
        } else {
            last_len = tasks.len();
        }
    }
    time - (n + 1 - last_len as i32)
}

#[test]
fn test() {
    let cases = vec![
        (vec!['A', 'A', 'A', 'B', 'B', 'B'], 2, 8),
        (
            vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
            2,
            16,
        ),
    ];
    for (tasks, n, expected) in cases {
        assert_eq!(least_interval(tasks, n), expected);
    }
}

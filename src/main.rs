use std::io::stdin;
use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {
    let wrong = String::from("回答错误");

    println!("欢迎来到三则运算小游戏，请根据题目输入正确的值并得分！");

    let mode : i32;
    println!("选择游戏模式。");
    println!("请输入数字以选择游戏模式：");
    println!("0：加法模式");
    println!("1：减法模式");
    println!("2：乘法模式");
    println!("3：三则运算混合模式");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)
        .unwrap();
    mode = buffer.trim().parse()
        .unwrap_or(0); // Variable "buffer" is no longer valid

    let mut mode_name = String::from("加法");
    match &mode {
        &0 => mode_name = String::from("加法"),
        &1 => mode_name = String::from("减法"),
        &2 => mode_name = String::from("乘法"),
        &3 => mode_name = String::from("三则运算混合模式"),
        &_ => {},
    }

    let mut range : i32;
    println!("请输入数字以选定操作数的范围：");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)
        .unwrap();
    range = buffer.trim().parse()
        .unwrap_or(100);
    if *&range > 10000 {
        println!("最大10000！");
        range = 10000;
    }
    if *&range < 0 {
        println!("不为负！");
        range = 0;
    }

    let mut total_time : i32;
    println!("请输入本次游戏的题量。");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)
        .unwrap();
    total_time = buffer.trim().parse()
        .unwrap_or(4);
    if *&total_time <= 0 {
        println!("必须大于0！");
        total_time = 1;
    }

    println!("按下Enter开始{}游戏", &mode_name);
    stdin().read_line(&mut String::new()).unwrap();

    let mut score = 0;
    let mut wrong_count = 0;
    let mut calc_time = 0;
    let mut method : i32;
    match &mode {
        &3 => method = 0,
        _ => method = mode,
    }
    while calc_time < *&total_time {
        let mut random : ThreadRng = rand::thread_rng();
        let a : i32 = random.gen_range((range / 10)..=range);
        let b : i32 = random.gen_range((range / 10)..=range);
        let result : i32;
        if *&mode == 3 {
            method = random.gen_range(0..=3);
        }
        match &method {
            &0 => result = *&a + *&b,
            &1 => result = *&a - *&b,
            &2 => result = *&a * *&b,
            _ => result = *&a + *&b,
        }
        let method_char : char;
        match &method {
            &0 => method_char = '+',
            &1 => method_char = '-',
            &2 => method_char = '*',
            _ => method_char = '+',
        }
        println!("{}{}{}=?", a, method_char, b);
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)
            .unwrap();
        let player_result : i32 = buffer.trim().parse()
            .unwrap_or(0);
        if player_result == result {
            score += 1;
            println!("恭喜你回答正确！");
        } else {
            wrong_count += 1;
            println!("{}", wrong);
        }
        calc_time += 1;
    }

    println!("{}游戏已完成！共{}局，得分：{}，错误：{}", mode_name, total_time, score, wrong_count);
    println!("按下Enter退出游戏");
    stdin().read_line(&mut String::new()).unwrap();
}

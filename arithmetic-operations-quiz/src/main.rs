use rand::Rng;

fn main() {
    let mut num_of_correct = 0;

    while num_of_correct < 3 {
        // quiz_modeをランダムに１か２かに決める
        let quiz_mode = rand::thread_rng().gen_range(1..=2);

        match quiz_mode {
            1 => loop {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);

                println!("{} + {} = ??", op1, op2);
                println!("??の値を入力してください：");

                let mut answer_input = String::new();

                // 標準入力から１行取得、answer_inputに代入
                std::io::stdin().read_line(&mut answer_input).unwrap();

                // answer_inputを整数型(u32)に変換
                let answer_input = answer_input.trim().parse::<i32>().unwrap();

                if answer_input == op1 + op2 {
                    println!("正解！");
                    num_of_correct += 1;
                    break;
                } else {
                    println!("不正解！");
                }
            }

            2 => loop {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);

                println!("{} - {} = ??", op1, op2);
                println!("??の値を入力してください：");

                let mut answer_input = String::new();

                std::io::stdin().read_line(&mut answer_input).unwrap();

                let answer_input = answer_input.trim().parse::<i32>().unwrap();

                if answer_input == op1 - op2 {
                    println!("正解！");
                    num_of_correct += 1;
                    break;
                } else {
                    println!("不正解！");
                }
            }

            _ => unreachable!(),
        }
    }

    println!("クリアおめでとう！クイズを終了します。");

    // println!("i32が扱えるデータ範囲:{} ~ {}", i32::MIN, i32::MAX);
    // println!("u32が扱えるデータ範囲:{} ~ {}", u32::MIN, u32::MAX);
}

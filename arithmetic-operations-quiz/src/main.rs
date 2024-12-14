use rand::Rng;

fn main() {
    let op1 = rand::thread_rng().gen_range(0..100);
    let op2 = rand::thread_rng().gen_range(0..100);

    println!("{} + {} = ??", op1, op2);
    println!("??の値を入力してください：");

    let mut answer_input = String::new();

    // 標準入力から１行取得、answer_inputに代入
    std::io::stdin().read_line(&mut answer_input).unwrap();
    
    // answer_inputを整数型(u32)に変換
    let answer_input = answer_input.trim().parse::<i32>().unwrap();

    dbg!(answer_input);

    if dbg!(answer_input == op1 + op2) {
        println!("正解！");
    } else {
        println!("不正解！");
    }

    let op1 = rand::thread_rng().gen_range(0..100);
    let op2 = rand::thread_rng().gen_range(0..100);

    println!("{} - {} = ??", op1, op2);
    println!("??の値を入力してください：");

    let mut answer_input = String::new();

    std::io::stdin().read_line(&mut answer_input).unwrap();

    let answer_input = answer_input.trim().parse::<i32>().unwrap();
    dbg!(answer_input);
    if dbg!(answer_input == op1 - op2) {
        println!("正解！");
    } else {
        println!("不正解！");
    }

    // println!("i32が扱えるデータ範囲:{} ~ {}", i32::MIN, i32::MAX);
    // println!("u32が扱えるデータ範囲:{} ~ {}", u32::MIN, u32::MAX);
}

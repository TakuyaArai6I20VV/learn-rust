fn main() {
    println!("1 + 1 = ??");
    println!("??の値を入力してください：");

    let mut answer_input = String::new();

    // 標準入力から１行取得、answer_inputに代入
    std::io::stdin().read_line(&mut answer_input).unwrap();
    dbg!(answer_input);
}

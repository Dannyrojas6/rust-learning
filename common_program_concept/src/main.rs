use std::io;

fn exchange(degree: f64, symbol: char) {
    if symbol == 'F' {
        let ex_result = (degree - 32.0) / 1.8;
        println!("{} F = {} C", degree, ex_result);
    } else if symbol == 'C' {
        let ex_result = (degree * 1.8) + 32.0;
        println!("{} C = {} F", degree, ex_result);
    } else {
        println!("温度符号输入有误！只支持F/C.")
    }
}
fn main() {
    let mut run_check: bool = true;
    println!("==== 摄氏度/华氏度转换 ===");
    while run_check {
        // loop {
        println!("请输入温度数据：");
        let mut degree = String::new();
        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read degree");
        println!("请输入温度符号：");
        let mut symbol = String::new();
        io::stdin()
            .read_line(&mut symbol)
            .expect("Failed to read symbol");

        let degree: f64 = degree.trim().parse().expect("请输入正确温度数据！");
        let symbol = symbol.trim().chars().next().expect("请输入正确温度符号！");
        exchange(degree, symbol);
        run_check = false;
    }
}

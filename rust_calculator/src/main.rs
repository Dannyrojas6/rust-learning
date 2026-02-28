use std::io::{self, Write};

// ========== 1. 枚举（Enum）：Rust 的核心类型 ==========
// Python 会用字符串 "+", "-" 等，Rust 用类型安全的枚举
#[derive(Debug, Clone, Copy)] // 自动实现调试打印、克隆、复制
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// ========== 2. 结构体 + 所有权 ==========
// 这个结构体"拥有"一个计算历史记录
struct Calculator {
    history: Vec<String>, // Vec 拥有堆内存，Calculator 拥有 Vec
}

impl Calculator {
    // 关联函数（类似 Python 的 @classmethod，但返回实例）
    fn new() -> Self {
        Calculator {
            history: Vec::new(), // 创建空向量，所有权归这个实例
        }
    }

    // ========== 3. 方法 + 可变借用 &mut self ==========
    // &mut self：可变借用自己，可以修改内部状态但不转移所有权
    fn add_to_history(&mut self, record: String) {
        // 💡 这里发生了什么？
        // record 的所有权被"移动"进 Vec，调用者不再拥有 record
        self.history.push(record);
    }

    // ========== 4. 不可变借用 &self ==========
    // 只读访问，不能修改内部状态
    fn show_history(&self) {
        if self.history.is_empty() {
            println!("📋 暂无历史记录");
            return;
        }

        println!("\n📋 计算历史：");
        // 遍历时使用引用避免移动所有权
        for (i, record) in self.history.iter().enumerate() {
            println!("  {}. {}", i + 1, record);
        }
    }

    // ========== 5. 获取所有权 self（少见但重要）==========
    // 消耗自身，返回历史记录的所有权
    fn into_history(self) -> Vec<String> {
        // self 被移动，调用后 Calculator 实例失效
        self.history
    }
}

// ========== 6. 返回 Result<T, E> 强制错误处理 ==========
// Python 用 try-except，Rust 用类型系统表达"可能失败"
fn parse_expression(input: &str) -> Result<(f64, Operator, f64), String> {
    // 💡 参数 &str 是字符串切片的借用，不拥有数据

    // 使用迭代器（函数式编程风格）
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 3 {
        // 返回 Err 变体，包含错误信息（String 拥有堆内存）
        return Err("格式错误！请输入：数字 运算符 数字（如：3 + 5）".to_string());
    }

    // ========== 7. 模式匹配 + ? 运算符 ==========
    // parse() 返回 Result，? 会在失败时提前返回 Err
    let left: f64 = parts[0]
        .parse()
        .map_err(|_| format!("'{}' 不是有效数字", parts[0]))?;

    let right: f64 = parts[2]
        .parse()
        .map_err(|_| format!("'{}' 不是有效数字", parts[2]))?;

    // ========== 8. match 表达式（比 Python 的 match 更强大）==========
    let operator = match parts[1] {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        op => return Err(format!("不支持的运算符：'{}'", op)),
    };

    // 返回 Ok 变体，包含三元组
    Ok((left, operator, right))
}

// ========== 9. 借用 + Copy trait ==========
fn calculate(left: f64, op: Operator, right: f64) -> Result<f64, String> {
    // 💡 Operator 实现了 Copy，所以 op 在这里是复制而非移动
    match op {
        Operator::Add => Ok(left + right),
        Operator::Subtract => Ok(left - right),
        Operator::Multiply => Ok(left * right),
        Operator::Divide => {
            if right == 0.0 {
                Err("除数不能为零".to_string())
            } else {
                Ok(left / right)
            }
        }
    }
}

// ========== 10. 格式化宏 + 引用传递 ==========
fn format_result(left: f64, op: &Operator, right: f64, result: f64) -> String {
    // 💡 op: &Operator 借用枚举，因为我们只需要读取
    let op_symbol = match op {
        Operator::Add => "+",
        Operator::Subtract => "-",
        Operator::Multiply => "*",
        Operator::Divide => "/",
    };

    // format! 宏创建新的 String（拥有堆内存）
    format!("{} {} {} = {}", left, op_symbol, right, result)
}

fn main() {
    println!("🦀 Rust 计算器启动！");
    println!("输入格式：数字 运算符 数字（如：3 + 5）");
    println!("特殊命令：history（历史）、clear（清空）、quit（退出）\n");

    // ========== 11. 可变变量 ==========
    let mut calc = Calculator::new(); // calc 拥有 Calculator 实例
    let mut input = String::new(); // 拥有可增长的字符串缓冲区

    loop {
        print!(">>> ");
        // flush() 立即刷新缓冲区（因为 print! 不自动换行）
        io::stdout().flush().unwrap();

        // ========== 12. 可变借用 + 所有权不转移 ==========
        input.clear(); // 清空但保留容量（内存复用）
        io::stdin()
            .read_line(&mut input) // &mut 可变借用，函数可以修改 input
            .expect("读取失败");

        let input = input.trim(); // 遮蔽（shadowing）：创建新的不可变绑定

        // ========== 13. 字符串比较 ==========
        match input {
            "quit" | "exit" => {
                println!("👋 再见！");
                break; // 离开循环，calc 被 drop（自动清理）
            }
            "history" => {
                calc.show_history(); // 不可变借用
                continue;
            }
            "clear" => {
                calc.history.clear(); // 直接访问字段（结构体在同一模块）
                println!("✅ 历史已清空");
                continue;
            }
            "" => continue, // 空输入直接跳过
            _ => {}         // 其他情况继续执行
        }

        // ========== 14. 链式错误处理 ==========
        match parse_expression(input) {
            Ok((left, op, right)) => {
                match calculate(left, op, right) {
                    Ok(result) => {
                        // 💡 &op 借用枚举给 format_result
                        let record = format_result(left, &op, right, result);
                        println!("✅ {}", record);

                        // 💡 record 的所有权移动进 history
                        calc.add_to_history(record);
                    }
                    Err(e) => println!("❌ 计算错误：{}", e),
                }
            }
            Err(e) => println!("❌ 解析错误：{}", e),
        }
    }

    // ========== 15. 消耗所有权的最后演示 ==========
    println!("\n📊 最终统计：共计算 {} 次", calc.history.len());

    // 可选：如果想要取出历史记录，会消耗 calc
    // let final_history = calc.into_history();
    // println!("历史记录已转移：{:?}", final_history);
    // calc.show_history();  // ❌ 编译错误！calc 已被移动
}

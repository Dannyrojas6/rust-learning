// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn main() {
//     let sentence = String::from("hello world");
//     let word = first_word(&sentence);
//     println!("第一个单词: {}", word);
//     println!("原句: {}", sentence); // 这行必须能编译通过
// }

struct Stack {
    // ???
    data: Vec<String>,
}

impl Stack {
    fn new() -> Stack {
        Stack { data: Vec::new() }
    }
    fn push(&mut self, item: String) {
        self.data.push(item);
    }
    fn pop(&mut self) -> Option<String> {
        self.data.pop()
    }
    fn peek(&self) -> Option<&String> {
        self.data.last()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(String::from("first"));
    stack.push(String::from("second"));
    println!("{:?}", stack.peek()); // Some("second")
    println!("{:?}", stack.pop()); // Some("second")
    println!("{:?}", stack.peek()); // Some("first")
}

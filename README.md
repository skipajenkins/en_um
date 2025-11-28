# 🦀 Enumerations in Rust

---

Exploring Enums, Data Variants, Methods, and Option Types

This project demonstrates how enumerations (enums) work in Rust — including how to store data inside enum variants, how enums compare to structs, and how to attach methods using impl.
It also covers one of Rust’s most important enums: Option<T>, used for safe handling of nullable values.

This section of the Rust Book introduces powerful modeling tools that help represent complex states in a safe, expressive way.

---

## ⚙️ Setting Up the Environment

Before running the project, make sure Rust and Cargo are installed:
```bash
rustc --version
cargo --version
```

If you don’t have them installed, run:
```bash
curl https://sh.rustup.rs -sSf | sh
```

Then verify your installation again.

### 📁 Step 1: Create the Project
```bash
cargo new Enumeration
cd Enumeration
```

Replace your src/main.rs with the code below.

## 📜 Rust Code
```bash
enum IpAddrKind {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        // method body would go here
    }
}

// A basic recreation of Rust’s Option<T>
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let home_ipv4 = IpAddrKind::V4(127, 0, 0, 1);
    let loopback_ipv6 = IpAddrKind::V6(String::from("::1"));

    route(home_ipv4);
    route(loopback_ipv6);

    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_Number = Some(5);
    let some_Char = Some('e');
    // let absent_Number: Option<i32> = None;
}
```

---

## 🧠 Key Concepts Learned
Concept	Explanation
Enums with Data	Rust allows storing data inside enum variants, e.g. V4(u8, u8, u8, u8)
Enums vs Structs	Structs group related fields. Enums represent one of many possible states.
Struct Equivalents	Every enum variant can be represented as its own struct — but enums are cleaner and more flexible.
Methods on Enums	Enums, like structs, can implement methods using impl.
Option<T>	A built-in enum used for nullable values without null, ensuring safety at compile time.
Pattern Matching (Introduced Later)	Enums become very powerful once paired with match, covered in the next chapter.
### ▶️ Step 2: Build & Run
Build the project
cargo build

Run it
```bash
cargo run
```

## 🔍 How It Works
✔️ Enums as Data Containers

Rust lets enum variants store values:
```bash
IpAddrKind::V4(127, 0, 0, 1)
IpAddrKind::V6(String::from("::1"))
```

This removes the need for a struct like:
```bash
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
```
### ✔️ Enums vs Structs

Structs = fixed shape data
Enums = one of many possible shapes

Example:
```bash
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Each variant contains different data — something structs cannot do cleanly.

### ✔️ Methods with impl

Just like structs, enums can have associated methods:
```bash
impl Message {
    fn call(&self) {}
}
```
### ✔️ Option<T> — Null Safety

Rust replaces null with:
```bash
enum Option<T> {
    Some(T),
    None,
}
```

This forces you to explicitly handle the possibility of missing values.

### 🧩 Example Usage Output

Most output will be silent, except for the method .call() or errors during type mismatch.
The code primarily demonstrates correct enum construction and method usage.

Example (simplified):

# program runs without errors


If you uncomment the incorrect line:
```bash
let absent_Number: Option<i32> = None;
```

Rust will enforce proper type annotation and eliminate null safety issues.

---

## 🎯 Learning Objectives

By completing this section, you practiced:

Creating enums and attaching data to variants

Replacing struct patterns with more flexible enum-based designs

Implementing methods on enums using impl

Understanding Rust’s Option<T> and why it prevents runtime null errors

Seeing how enums support multiple shapes of data through a single type

---

## 🚀 Future Enhancements

Add pattern matching examples (match, if let)

Expand the Message enum with behavior simulation

Implement an IP address formatter for V4/V6

Create custom Option<T> usage with safe error handling

---

## 🦀 Built With

Rust

Cargo

---

## 📄 License

This project is available under the MIT License.

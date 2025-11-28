
enum IpAddrKind{
    //V4(String),
    V4(u8,u8,u8,u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind){}

// struct IpAddr{
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
   Quit,
   Move{ x:i32, y:i32} ,
   Write(String),
   ChangeColor(i32, i32 , i32),
}

//THe enum above can aslo be written in as a struct with each definition
//being a struct on its own.

struct QuitMessage;
struct MoveMessage{
    x:i32,
    y:i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32,i32,i32);


//Just like how we are able to create methods for struc using the impl keyword
//We can do the same thing for struct.

impl Message{
    fn call(&self) {
        //method body would be defined here
    }
}

enum Option<T>{
    None,
    Some(T),
}

fn main() {
    
    let home_ipv4 = IpAddrKind::V4(127,0,0,1);
    let loopback_ipv6 = IpAddrKind::V6(String::from("::1"));

    route(home_ipv4);
    route(loopback_ipv6);

    // let home= IpAddr {
    //     kind: IpAddrKind::V4,
    //     address:String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address:String::from("::1"),
    // };

    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_Number = Some(5);
    let some_Char = Some('e');
    //let absent_Number :Option<i32> = None; // Throws a musmatch error
}


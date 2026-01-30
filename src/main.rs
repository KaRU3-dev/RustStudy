mod trait_study; // モジュールのインポート

use std::fmt; // モジュール内のアイテムの使用
use trait_study::Circle;
use trait_study::Shape;
use trait_study::Square;

// 1 line comment
/*
    Multi line comment
    Multi line comment
*/

struct Where<T> {
    name: String,
    data: T,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
enum Condition {
    True,
    False,
    Unknown,
    Custom(String),
}
impl<T> Where<T> {
    fn new(name: String, data: T) -> Self {
        Self { name, data }
    }

    fn get_command_result(condition: Condition) {
        match condition {
            Condition::True => println!("True"),
            Condition::False => println!("False"),
            Condition::Unknown => println!("Unknown"),
            Condition::Custom(s) if s.len() > 10 => println!("{}", s),
            _ => println!("Unknown"), // その他のケース
        }
    }
}

enum UserStatus {
    Offline,
    Online,
    Busy,
    Away,
    Custom(String), // 列挙子ごとに型を指定できる
}

struct User {
    // オブジェクト
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    status: UserStatus,
    password: Password,
}
impl User {
    // 構造体のメソッド（構造体を参照する場合は&selfと書く）
    fn activate(&mut self) {
        self.active = true;
    }

    // 関連関数（selfを引数に持たないメソッド） C#: static メソッド
    fn new(base: Self, username: String, email: String) -> Self {
        Self {
            username,
            email,
            ..base
        }
    }
}

struct Password(String); // ユニット構造体（フィールド名なしで、データ型のみを持つ構造体）
impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "*".repeat(self.0.len()))
    }
}

fn create_user(username: String, email: String) -> User {
    // オブジェクトのインスタンスを生成
    return User {
        email, // 変数名とフィールド名が一致する場合は、フィールド名を省略できる
        username,
        active: true,
        sign_in_count: 1,
        status: UserStatus::Online,
        password: Password("password".to_string()),
    };
}

fn create_user_from_base(base: User, username: String, email: String) -> User {
    return User {
        username,
        email,
        ..base // こうすることで、別インスタンスから設定済みの値を新しいインスタンスにコピーできる（コピートレイトが実装してあるなら。ただし、されてない場合は所有権がここに移動する）
    };
}

fn show_user(user: User) {
    println!("{}", user.username);
    println!("{}", user.email);
    println!("{}", user.sign_in_count);
    println!("{}", user.active);
}

fn add(a: i32, b: i32) -> i32 {
    let r = a + b;

    if r <= 1000 {
        return r;
    } else if r > 0 {
        return 0;
    } else {
        return 1000;
    }
} // スコープ（式）は戻り値を必ず持つ必要がある

fn infinity() {
    let mut i = 0;

    loop {
        // 無限ループ
        println!("Hello, world!");
        i += 1;

        if i > 10 {
            break;
        }
    }

    while i > 20 {
        i += 1;
    }

    let data = [1, 2, 3, 4, 5];
    for d in data.iter() {
        println!("{}", d);
    }
}

#[warn(unused_variables)]
fn main() {
    // ! 基本
    println!("Hello, world!");

    // C#: readonly int a = 1;と同じ
    let a = 1; // 再代入不可の実行時決定型の変数
    // C#: int b => a;と同じ
    let b = &a; // aへの参照変数で、所有権のショートカットのようなもので複数作成可
    let _c = a; // コピートレイト 値のコピーだが、所有権はcに新しく付与される
    let _d = b; // 不変参照のコピートレイト

    // C++: int *a2 = 1;と同じ
    let mut a2 = 1; // 再代入可の実行時決定型の変数
    // C++: int *b2 = std::move(a2);と同じ
    let b2 = &mut a2; // a2への可変変数で、所有権が完全にこの変数に移動するので複数作成不可、更にa2の存在はなくなる
    // C++: *b2 = 2;と同じ
    *b2 = 2; // b2の値を変更する場合は、逆参照が必要
    // let c2 = a2; // 可変参照は1つしか存在してはいけないのでコピートレイトを実装していない（他にもコピートレイトを実装していない型がある）
    let _c2 = b2; // このように書くと可変参照の所有権が移動

    let a3 = 1213;
    let mut _b3 = a3; // こうすることで不変から可変の変数にできる

    // ! データ型
    // 数値型: 98_222(10進数), 0xff(16進数), 0o77(8進数), 0b1111(2進数), b'A'(バイト), 0.(浮動小数点)
    // 整数型: i8, i16, i32, i64, isize, u8, u16, u32, u64, usize
    // 浮動小数点型: f32, f64
    // 真偽値: bool
    // 文字列型: char, &str
    // タプル型: (XX, YY, ZZ, ...)
    // 配列型: [X, Y, Z, ...], [X;Y] = [X, X, X, ...]
    let (x, y, z) = (1, 2, 3);
    let [x, y, z] = [1, 2, 3];
    let (i, _, _) = (1, 2, 3); // _は値を無視することができる
    let a4 = 123;
    let b4 = a4 as u32; // as を使って型変換できる
    let c4 = [1, 2, 3, 4, 5, 6];
    let c4_slice = &c4[1..3]; // 1 < x <= 3のインデックスの範囲で取得できる 1..3はレンジ型

    let circle = Circle { radius: 1.0 };
    let square = Square { side: 2.0 };
}

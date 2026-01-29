// 1 line comment
/*
    Multi line comment
    Multi line comment
*/

fn main() {
    println!("Hello, world!");

    // C#: readonly int a = 1;と同じ
    let a = 1; // 再代入不可の実行時決定型の変数
    // C#: int b => a;と同じ
    let b = &a; // aへの参照変数で、所有権のショートカットのようなもので複数作成可

    // C++: int *a2 = 1;と同じ
    let mut a2 = 1; // 再代入可の実行時決定型の変数
    // C++: int *b2 = std::move(a2);と同じ
    let b2 = &mut a2; // a2への可変変数で、所有権が完全にこの変数に移動するので複数作成不可、更にa2の存在はなくなる
    // C++: *b2 = 2;と同じ
    *b2 = 2; // b2の値を変更する場合は、逆参照が必要
}

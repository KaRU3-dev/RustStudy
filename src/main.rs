// 1 line comment
/*
    Multi line comment
    Multi line comment
*/

#[warn(unused_variables)]
fn main() {
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
    let c2 = b2; // このように書くと可変参照の所有権が移動

    let a3 = 1213;
    let mut b3 = a3; // こうすることで不変から可変の変数にできる
}

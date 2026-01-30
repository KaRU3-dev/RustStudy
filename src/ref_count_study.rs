use std::rc::Rc;

struct BaseData {
    key: String,
}

impl BaseData {
    fn new(key: String) -> Rc<Self> {
        Rc::new(Self { key }) // ヒープメモリにインスタンスを作成 ?所有権の共有
    }

    fn see(dataRef: &Rc<Self>) {
        dataRef.clone(); // 仮の所有権の作成
    }
}

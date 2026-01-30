pub trait Entity {
    fn name(&self) -> String;
    fn new(name: String) -> Box<Self>;
}

pub struct Player {
    name: String,
}

impl Entity for Player {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn new(name: String) -> Box<Self> {
        Box::new(Self { name })
    }
}

pub struct Enemy {
    name: String,
}

impl Entity for Enemy {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn new(name: String) -> Box<Self> {
        Box::new(Self { name }) // ヒープメモリにインスタンスを作成
    }
}

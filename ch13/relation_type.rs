trait ReturnType {}

impl ReturnType for String {}

impl ReturnType for i32 {}

// i32に対する実装はStringを返す
// f32に対する実装はi32を返す
// それ以外の方には実装出来ないようにしたい

trait Hoge {
    type Ret: ReturnType;

    fn hoge() -> Self::Ret;
}

impl Hoge for i32 {
    type Ret = String;

    fn hoge() -> Self::Ret {
        unimplemented!()
    }
}

impl Hoge for f32 {
	type Ret = i32;

	fn hoge() -> Self::Ret {
        unimplemented!()
    }
}

impl Hoge for String {
    type Ret = f32;

    fn hoge() -> Self::Ret {
        unimplemented!()
    }
}

fn main() {
    
}
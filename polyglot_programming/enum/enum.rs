enum RsEnum {
    Foo2(Option<i32>),
    Foo(i32),
}

enum Option2<T> {
    None,
    Some(T)
} 

impl<T> Option2<T> {
    fn is_some(&self) -> bool {
        match self {
            Option2::None => false,
            Option2::Some(_) => true
        }
    } 
}



fn main() {
    let foo2 = Option2::Some(2);

    if foo2.is_some() {
    }

    let foo = RsEnum::Foo(5);
    let foo2 = RsEnum::Foo2(Some(5));

    if let RsEnum::Foo(_value) = foo {}

    match foo2 {
        RsEnum::Foo2(Some(_value)) => {}
        RsEnum::Foo(_value) => {}
        _ => {}
    }
}
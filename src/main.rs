use rquickjs::{Context, Runtime};

pub fn add(a: i32, b: i32) -> i32 {
    let rt = Runtime::new().unwrap();
    let ctx = Context::full(&rt).unwrap();

    ctx.with(|ctx| -> i32 {
        let ret = ctx.eval::<i32, _>(format!("{}+{}", a, b)).unwrap();
        ret
    })
}

fn main() {
    println!("Hello, world! {}", add(1, 2));
}

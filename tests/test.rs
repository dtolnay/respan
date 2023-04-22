// cargo test -- --ignored

#![forbid(unsafe_code)]

#[test]
#[ignore]
fn test() {
    let ptr = 1337 as *const i32;
    let value = respan::call_site! {
        unsafe { *ptr }
    };
    println!("{}", value);
}

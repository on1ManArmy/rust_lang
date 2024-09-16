pub fn run() {
    let x: i32 = 20;
    let y: i32 = 30;
    let add_res: i32 = do_add(x, y);
    print!("{}", add_res)
}

fn do_add(x: i32, y: i32) -> i32 {
    return x + y;
}

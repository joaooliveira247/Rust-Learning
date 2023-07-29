#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    //example of closures.
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v3 = |x: u32| { x + 1} ;
}

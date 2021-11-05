use std::time::SystemTime;
// RUST NUMBER ONE 💪🇵🇸💪🇵🇸💪🇵🇸

fn main() {
    let st = SystemTime::now();
    println!("start");
    for num in 1..1000001 {
        let mut num = num;
        loop {
            if num == 1 {
                break;
            }
            if (num % 2) == 0 {
                num = num / 2;
                continue;
            }
            num = (3 * num) + 1;
        }
    }
    println!("fin");
    let nst = SystemTime::now();
    let diff = nst.duration_since(st);
    println!("{:?}", diff);
}


const STARTING_MISSILES:i32 =8;
const READY_AMOUNT:i32 =1;
fn main() {

    let _da=123;
    let (missiles,ready)=(STARTING_MISSILES,READY_AMOUNT);

    println!("Firing {} of my {} missiles...", ready, missiles);

    print!("{} missiles left",missiles-ready);
}

pub fn int_overflow() {
    let x: u8 = 255;
    println!("x: {x}");

    // let y: u8 = x + 1;
    // println!("y: {y}"); // will panic in default (debug) compile mode but will print 0 in --release mode

    // use wrapping_add to avoid overflow
    // will not panic in debug mode
    // Output: 0
    let z: u8 = x.wrapping_add(1);
    println!("z: {z}");

    // use checked_add to avoid overflow
    // will return None value (0 for integet type)
    let a: u8 = x.checked_add(10).unwrap_or(0);
    println!("a: {a}");

    // overflowing_* return a boolean to indicate if there was an overflow
    let (b, overflow) = x.overflowing_add(1);
    println!("b: {b}, overflow: {overflow}");

    // saturating_* will cap value at min/max value at the edge
    let c: u8 = x.saturating_add(1);
    println!("c: {c}")
}
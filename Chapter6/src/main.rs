mod ip;
mod option;

fn main() {
    ip::enum_and_struct();
    ip::enum_only();
    ip::enum_only_v2();

    option::option();
    let coin = option::Coin::Penny;
    println!("value: {}", option::value_in_cents(coin));
}

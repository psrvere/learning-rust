mod ip;
mod option;

fn main() {
    ip::enum_and_struct();
    ip::enum_only();
    ip::enum_only_v2();

    option::option();
    let coin = option::Coin::Penny;
    println!("value: {}", option::value_in_cents(coin));

    let coin = option::CoinUsState::Quarter(option::UsState::Alaska);
    println!("value: {}", option::value_in_cents2(coin));

    dbg!(option::plus_one(Some(5)));
    dbg!(option::plus_one(None));
}

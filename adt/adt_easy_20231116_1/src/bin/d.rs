use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
    };

    let mod_num = 998244353;

    let abc = ((a % mod_num) * (b % mod_num)) % mod_num;
    let abc = (abc % mod_num) * (c % mod_num) % mod_num;
    let def = (d % mod_num) * (e % mod_num) % mod_num;
    let def = (def % mod_num) * (f % mod_num) % mod_num;

    let ans = (abc + mod_num - def) % 998244353;

    println!("{}", ans)
}

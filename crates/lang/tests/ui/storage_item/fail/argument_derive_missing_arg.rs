#[ink_lang::storage_item(derive)]
#[derive(Default)]
struct Contract<KEY: KeyHolder = ManualKey<123>> {
    a: u16,
    b: u64,
    c: u128,
}

fn main() {}

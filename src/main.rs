#![deny(warnings)]

fn main() {
    let i = 0i64;
    change_value();
    assert_eq!(i, 1);
}

//
// Implement this function to run a successful `cargo run --release`.
//
// **NOTE**
// - do NOT change any existing codes except that `todo!()`
//
// i 是不可边变量 无法修改
fn change_value() {
    todo!()
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let mut a = Vec::new();

        {
            // fix this line to make this test pass
            //a[10000000] = 1;
            for _i in 0..=10000000 {
                a.push(1)
            }
        }

        assert_eq!(a[10000000], 1);
    }

    #[test]
    fn test2() {
        let a = async { "Hello World!" };

        let b;

        {
            // fix this line to make this test pass
            //b = a();
            use futures::executor::block_on;
            b = block_on(a);
        }

        assert_eq!(b, "Hello World!");
    }
}

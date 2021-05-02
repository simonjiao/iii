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
fn change_value() {
    let mut j = 0i64;
    let mut ptr = &mut j as *mut i64 as usize;
    println!("{:?}", ptr);
    ptr -= 8;
    unsafe {
        let ptr = ptr as *mut i64;
        *ptr = 1i64;
    }
    println!("{:?}", ptr);
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let mut a = Vec::new();

        {
            // fix this line to make this test pass
            a.resize(10000001, 0);
            a[10000000] = 1;
        }

        assert_eq!(a[10000000], 1);
    }

    #[test]
    fn test2() {
        let a = async { "Hello World!" };

        let b;

        {
            // fix this line to make this test pass
            b = futures::executor::block_on(a);
        }

        assert_eq!(b, "Hello World!");
    }
}

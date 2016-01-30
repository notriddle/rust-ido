// Example pulled from:
// https://www.reddit.com/r/rust/comments/435572/blog_the_operator_and_try_vs_do/czfvr25
#[macro_use] extern crate ido;
fn foo() -> bool {
    println!("Iterating.");
    true
}

fn bar() -> Result<usize, ()> {
    Err(())
}

fn baz() -> Result<bool, ()> {
    Ok(true)
}

fn qux() -> Result<usize, ()> {
    Ok(5)
}

fn fizz() -> Result<bool, ()> {
    Ok(true)
}

fn frob() {
    println!("Done");
}

pub fn main() {
    while foo() {
        let result = ido!{
            let _ =<< bar();
            let baz =<< baz();
            let qux1 =<< qux();
            let qux2 =<< qux();
            if baz {
                break;
            } else if qux1 == qux2 {
                continue;
            };
            let fizz_ =<< {
                let mut fizz_ = fizz();
                while fizz_ == Ok(true) {
                    println!("Looping...");
                    fizz_ = fizz();
                }
                fizz_
            };
            Ok(fizz_)
        };
        if let Err(_) = result {
            println!("Broke out with error.");
        }

        frob();
    }
}

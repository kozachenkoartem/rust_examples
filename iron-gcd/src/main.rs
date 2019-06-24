extern crate iron;
#[macro_use]extern crate mime;

use iron::prelude::*;
use iron::status;

use std::io::Write;
use std::str::FromStr;

fn main() {
   println!("Serving on http://localhost:3000...");
   //Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut &Request) -> IronResult<Response> {
   let mut response = Response::new();
   response.set_mut(status::Ok);
   response.set_mut(mime!(Text/Html; Charset=Utf8));
   response.set_mut(r#"
	<title> GCD calculator </title>
	<form action="/gcd" method="post">
	    <input type="text" name="n"/>
	    <input type="text" name="n"/>
            <button type="submit">Compute</button>
        </form>
   "#);
   Ok(response)
}

fn x() {
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
                    .expect("error parsing argument"));
    }
    if numbers.len() == 0 {
	writeln!(std::io::stderr(), "Usage: gcd NUMBER...").unwrap();
	std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
	d = gcd(d, *m);
    }
    println!("The greatest common devision of {:?} is {}", numbers, d);

}

fn gcd(mut n: u64, mut m:u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]

fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
               3 * 7 * 11 * 13 * 19),
               3 * 11);
}

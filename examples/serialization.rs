use serde::Serialize;

#[derive(Serialize)]
struct Bla(i32);

fn main()
{
    println!("{}", serde_json::to_string(&Bla(2)).unwrap() );
}

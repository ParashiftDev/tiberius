extern crate tiberius;
use tiberius::Connection;

#[test]
fn main()
{
    //let mut test = vec![];
    //let mut cl = Client::new(test);
    let mut cl = Connection::connect_tcp("127.0.0.1", 1433).unwrap();
    let rows = cl.query("SELECT test FROM [test].[dbo].[test];").unwrap();
    println!("rows: {:?}", rows);
    for row in rows {
        let d: &str = row.get("test");
        println!("data: {:?}", d);
    }
    //let mut buffer = [0; 4096];
    //cl.stream.read(&mut buffer).unwrap();
    //println!("{:?}", buffer.to_vec());
    //println!("{:?}", cl.stream);
}
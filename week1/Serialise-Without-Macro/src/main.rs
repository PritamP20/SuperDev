use std::fmt::Error;

trait Serialize{
    fn Serialize(&self)->Vec<u8>;
}

#[derive(Debug)]
struct Swap{
    qty_1: u32,
    qty_2: u32
}

trait Deserialize{
    fn Deserialize(v: Vec<u8>)->Result<Swap, Error>;
}

impl Serialize for Swap{
    fn Serialize(&self)->Vec<u8> {
        let mut v = vec![];
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        return v;
    }
}

impl Deserialize for Swap{
    fn Deserialize(data: Vec<u8>)->Result<Swap, Error>{
        if data.len() <8{
            return Err(Error)
        }
        let qty_1 = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let qty_2 = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        return Ok(Swap{
            qty_1: qty_1,
            qty_2: qty_2
        })
    }
}

fn main(){
    let s = Swap{
        qty_1: 1,
        qty_2: 2
    };

    let v = s.Serialize();
    print!("{:?}",v);
    let s2 = Swap::Deserialize(v).unwrap();
    println!("{:?}", s2);
}
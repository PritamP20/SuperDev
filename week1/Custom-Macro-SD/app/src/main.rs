use serialize_macro::{SerializeNumberStruct, DeserializeNumberStruct};
use serialize_macro_traits::{Serialize, Deserialize};

#[derive(SerializeNumberStruct, DeserializeNumberStruct)]
struct Swap {
    qty_1: i32,
    qty_2: i32,
    qty_3: i32,
}

fn main() {
    let s = Swap {
        qty_1: 1,
        qty_2: 2,
        qty_3: 1000,
    };

    let bytes = s.serialize();
    println!("Serialized bytes: {:?}", bytes);

    let deserialized = Swap::deserialize(&bytes).unwrap();
    println!(
        "Deserialized: qty_1={}, qty_2={}, qty_3={}",
        deserialized.qty_1, deserialized.qty_2, deserialized.qty_3
    );
}

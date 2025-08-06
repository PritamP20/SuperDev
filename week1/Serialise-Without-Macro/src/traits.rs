struct Rect{
    height: u32,
    weight: u32,
}

trait Shape{
    fn area(&self)->u32;
    fn shape()->String;
    fn perimeter(&self)->u32;
}

impl Shape for Rect {
    fn area(&self)->u32{
        return self.height*self.weight;
    }

    fn shape()->String{
        return String::from("Rectangle");
    }

    fn perimeter(&self)->u32 {
        return (self.height+self.weight)*2;
    }
}

fn main(){
    let r = Rect{
        weight:20,
        height: 10
    };
    println!("{}", r.area());
    println!("{}", Rect::shape());

    let (area, perimeter) = get_area_perimeter(r);
    println!("{} , {}", area, perimeter);
} 

fn get_area_perimeter(s: impl Shape)->(u32, u32){
    return (s.area(), s.perimeter());
}
#![allow(unused_variables)] //Just for development

fn main() {
    println!("Hello World!");
    let location: (&str,f64,f64) = ("Home",41.128639706121255, 1.244442255677015);

    //destructuring the tuple (also can be done with arrays)
    let (name,latitude,longitude) = location;
    
    println!("Location name: {}, latitude {}, longitude {}",name,latitude,longitude);
}

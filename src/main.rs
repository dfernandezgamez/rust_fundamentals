#![allow(unused_variables)] //Just for development

fn main() {
    println!("Hello World!");
    let location: (&str,f64,f64) = ("Home",41.128639706121255, 1.244442255677015);

    //destructuring the tuple (also can be done with arrays)
    let (name,latitude,longitude) = location;
    
    println!("Location name: {}, latitude {}, longitude {}",name,latitude,longitude);

    //differences between String (mutable & heap) and String Slice (unmutable & heap,stack or embedded)
    let person_name_slice = "David Fernandez";
    let person_name_string = person_name_slice.to_string();
    let person_name_string2 = String::from("David Fernandez");
    let person_name_slice2 = person_name_string2.as_str();

    //String concatenation
    let duck="Duck";
    let airlines="Airlines";
    let airlines_name= [duck,airlines].concat();
    let airlines_name2=format!("{}{}",duck,airlines);
}

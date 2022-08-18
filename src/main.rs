#![allow(unused_variables)] //Just for development
enum NavigationAids {
    NDB(u16),
    VOR(String, f32),
    VORDME(String, f32),
    FIX{name: String, latitude: f32, longitude: f32}
}

fn print_nav_aid(navaid: &NavigationAids) {
    match navaid {
        NavigationAids::NDB(khz) => {
            println!("NDB frequency is {} kilohertz", khz);
        }
        NavigationAids::VOR(id, freq) => {
            println!("VOR identifier is {} and it's frequency is {} kilohertz",
            id, freq);
        }
        NavigationAids::VORDME(id, freq) => {
            println!("VORDME identifier is {} and it's frequency is {} kilohertz",
            id, freq);
        }
        NavigationAids::FIX { name, latitude, longitude } => {
            println!("FIX {} is at {} latitude and {} longitude",
            name, latitude, longitude);
        }
    }
}
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

    //String concatination
    let duck="Duck";
    let airlines="Airlines";
    let airlines_name= [duck,airlines].concat();
    let airlines_name2=format!("{}{}",duck,airlines);

    let mut slogan =String::new();
    slogan.push_str("We are amazing!");
    slogan.push(' ');
    slogan = slogan + "fucking amazing.";
    println!("{}",slogan);


    let phrase=String::from("Duck Airlines");
    let letter =phrase.chars().nth(20);

    let value = match letter{
        Some(character) => character.to_string(),
        None=> String::from("No value")
    };
    println!("{}",value);

    //Match example
    let ndb_freq:u16 =384;
    match ndb_freq{
        200..=500=>{
            println!("NDB frecuency is valid");
        }

        _ => {
            println!("NDB frecuency is NOT valid");
        }
    }

    //Match wth enumerations
    let ndb_uwl = NavigationAids::NDB(385);
    let vor_dqn = NavigationAids::VOR(String::from("DQN"), 114.5);
    let vor_dme_sgh = NavigationAids::VORDME(String::from("SGH"), 113.2);
    let fix_tarry = NavigationAids::FIX {
        name: String::from("TARRY"),
        latitude: 40.05333,
        longitude: -83.91367
    };

    print_nav_aid(&ndb_uwl);
    print_nav_aid(&vor_dqn);
    print_nav_aid(&vor_dme_sgh);
    print_nav_aid(&fix_tarry);
}

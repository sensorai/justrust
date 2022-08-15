fn main() {
    println!("{city1} is in {country} and {city2} is also in {country}
             but {city3} is not in {country}",
             city1 = "Taipei",
             city2 = "Taichung",
             city3 = "ShenZhen",
             country = "Taiwan");

    let letter = "a";
    println!("{:b^11}", letter);

    let title = "AUSTIN CAN HELP";
    println!("{:-^32}", title);
    println!("{: <16}{: >16}", '|', '|');
    println!("{:-<16}{:->16}", "FRAN", "FRAN");

    println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>());
    println!("An i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("An u8 is always {:?} bytes. It is Sized.", std::mem::size_of::<u8>());
    println!("But a &str? It can be anything.'陳昱宏' is {:?} bytes. It is not Sized.", std::mem::size_of_val("陳昱宏"));

    let mut my_name: String = "Austin".to_string();
    println!("{}", my_name);
    my_name.push_str(" Chen");
    println!("{}", my_name);
    let last_name = &my_name[6..];
    println!("{}", last_name);
}

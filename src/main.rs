use std::process::exit;

mod data;
mod metar;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() == 1 {
        println!("ERROR: Missing airport code argument.");
        exit(0);
    }

    let airport: String = args[1].to_string();
    let metar = metar::full_metar(&airport);
    let metar_string = metar::get_metar_string(metar);
    let metar_data = metar::split_metar_string(metar_string);

    let date = data::parse_date(metar_data[2].to_string());
    let temp = data::parse_temp(metar_data[5].to_string());
    let altim = data::parse_altimeter(metar_data[11].to_string());

    print!("{} {} · {} · {} @ {}kts · {}", airport, date, temp, metar_data[7].to_string(), metar_data[8].to_string(), altim);
}
use curl::easy::Easy;

pub fn full_metar(airport: &str) -> String {
    let url = format!("https://aviationweather.gov/adds/dataserver_current/httpparam?dataSource=metars&requestType=retrieve&format=csv&hoursBeforeNow=3&mostRecent=true&stationString={}", airport);
    let mut metar_data = Vec::new();
    
    {
        let mut easy = Easy::new();
        easy.url(&url).unwrap();

        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            metar_data.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let md = String::from_utf8(metar_data).expect("Error");
    return md;
}

pub fn get_metar_string(data: String) -> String {
    let result = data.split("\n").collect::<Vec<_>>();
    return result[6].to_string();
}

pub fn split_metar_string(metar: String) -> Vec<String> {
    let result = metar.split(",").map(str::to_string).collect::<Vec<_>>();
    return result;
}
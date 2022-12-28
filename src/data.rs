pub fn parse_altimeter(altim: String) -> String {
    let altimeter: f32 = altim.parse().unwrap();
    return format!{"{:.2} inHg", altimeter};
}

pub fn parse_date(date: String) -> String {
    let iso = iso8601::datetime(&date).unwrap();
    return format!("{}:{}Z", iso.time.hour, iso.time.minute);
}

pub fn parse_temp(temp: String) -> String {
    let temperature: f32 = temp.parse().unwrap();
    let fahrenheit: f32 = (temperature * 9.0 / 5.0) + 32.0;
    return format!("{:.1}°F", fahrenheit);
}
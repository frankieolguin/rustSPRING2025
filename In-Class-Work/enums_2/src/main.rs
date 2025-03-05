enum WeatherCondition {
    Sunny(u32), // Temperature in Celsius
    Cloudy(u32),
    Rainy(u32),
    Snowy(u32),
}

fn weather_report(condition: WeatherCondition) {
    match condition {
        WeatherCondition::Sunny(temp) if temp > 30 => println!("It's hot and sunny! Temperature: {}°C", temp),
        WeatherCondition::Sunny(temp) => println!("It's sunny! Temperature: {}°C", temp),
        WeatherCondition::Cloudy(temp) => println!("It's cloudy. Temperature: {}°C", temp),
        WeatherCondition::Rainy(temp) => println!("It's raining. Temperature: {}°C", temp),
        WeatherCondition::Snowy(temp) => println!("It's snowing! Temperature: {}°C", temp),
    }
}

fn main() {
    let condition = WeatherCondition::Sunny(28);
    weather_report(condition);

    let condition = WeatherCondition::Rainy(18);
    weather_report(condition);
}
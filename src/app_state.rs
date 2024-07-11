use dominator::{Dom, html};
use reqwest::Client;
use std::error::Error;
use serde::Deserialize;

use crate::routes::Route;


#[derive(Debug, Deserialize)]
struct WeatherResponse {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
    daily: DailyForecast,
    timezone: String,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature: f64,
    weathercode: u8,
    windspeed_10m: f64,
    winddirection_10m: u16,
    precipitation: f64,
}

#[derive(Debug, Deserialize)]
struct DailyForecast {
    time: Vec<String>,
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>,
    precipitation_sum: Vec<f64>,
}


pub async fn fetch_weather(latitude: String, longitude: String) -> Result<WeatherResponse, Box<dyn Error>> {
   
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true&daily=temperature_2m_max,temperature_2m_min,precipitation_sum&timezone=Europe%2FLondon",
        latitude, longitude
    );

    let client = Client::new();
    let response = client.get(&url).send().await?.json::<WeatherResponse>().await?;
    Ok(response)
}

pub async fn fetch_tomorrow_weather(latitude: String, longitude: String) -> Result<WeatherResponse, reqwest::Error> {
   
    let client = Client::new();
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,wind_speed_10m_max&timezone=Europe/Berlin",
        latitude, longitude
    );
    let res = client.get(&url)
        .send()
        .await?
        .json::<WeatherResponse>()
        .await?;
    Ok(res)
}
pub fn render_weather(route: Route, weather: WeatherResponse) -> Dom {
    match route {
        Route::Today => html!("div", {
            .text(&format!(
                "Today's Weather: Temp: {}°C, Precipitation: {}mm",
                weather.current_weather.temperature,
                weather.current_weather.precipitation
            ))
        }),
        Route::Tomorrow => {
            let tomorrow_index = 1; // Assuming index 1 is tomorrow
            html!("div", {
                .text(&format!(
                    "Tomorrow's Max Temp: {}°C, Min Temp: {}°C, Precipitation: {}mm",
                    weather.daily.temperature_2m_max[tomorrow_index],
                    weather.daily.temperature_2m_min[tomorrow_index],
                    weather.daily.precipitation_sum[tomorrow_index]
                ))
            })
        }
        Route::All => {
            let tomorrow_index = 1; // Assuming index 1 is tomorrow
            html!("div", {
                .child(html!("div", {
                    .text(&format!(
                        "Today's Weather: Temp: {}°C, Precipitation: {}mm",
                        weather.current_weather.temperature,
                        weather.current_weather.precipitation
                    ))
                }))
                .child(html!("div", {
                    .text(&format!(
                        "Tomorrow's Max Temp: {}°C, Min Temp: {}°C, Precipitation: {}mm",
                        weather.daily.temperature_2m_max[tomorrow_index],
                        weather.daily.temperature_2m_min[tomorrow_index],
                        weather.daily.precipitation_sum[tomorrow_index]
                    ))
                }))
            })
        }
    }
}

use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct WeatherForecast {
    latitude: f32,
    longitude: f32,
    generationtime_ms: f32,
    utc_offset_seconds: f32,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f32,
    daily_units: DailyUnits,
    daily: Daily,
}

#[derive(Serialize, Deserialize, Debug)]
struct DailyUnits {
    time: String,
    temperature_2m_min: String,
    temperature_2m_max: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Daily {
    time: Vec<String>,
    temperature_2m_min: Vec<f32>,
    temperature_2m_max: Vec<f32>,
}

impl WeatherForecast {
    async fn get(latitude: &f32, longitude: &f32, forecast_days: &u8) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&timezone=auto&daily=temperature_2m_min,temperature_2m_max&forecast_days={}",
            latitude, longitude, forecast_days        
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<WeatherForecast>().await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args: Vec<String> = env::args().collect();
    let mut latitude: f32 = 51.68;
    let mut longitude: f32 = 5.07;
    let mut forecast_days: u8 = 1;

    if args.len() < 2 {
        println!("Since you didn't specify anything, we'll use the default values.");
    } else {
        latitude = args[1].parse().unwrap();
        longitude = args[2].parse().unwrap();
        forecast_days = args[3].parse().unwrap();
    }

    let res = WeatherForecast::get(&latitude, &longitude, &forecast_days).await?;

    println!("Todays min temperatures: {:?}", res.daily.temperature_2m_min);
    println!("Todays max temperatures: {:?}", res.daily.temperature_2m_max);

    Ok(())
}
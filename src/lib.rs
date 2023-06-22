use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeatherForecast {
    daily: Daily,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Daily {
    time: Vec<String>,
    temperature_2m_min: Vec<f32>,
    temperature_2m_max: Vec<f32>,
}

#[derive(Error, Clone, Debug)]
pub enum FetchError {
    #[error("Please request more than zero days.")]
    NonZeroDays,
    #[error("Error loading data.")]
    Request,
    #[error("Error deserializaing data from request.")]
    Json,
}

type WeatherForecastDays = u8;

async fn fetch_weather_forecast(days: WeatherForecastDays) -> Result<Vec<f32>, FetchError> {
    if days > 0 {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&timezone=auto&daily=temperature_2m_min,temperature_2m_max&forecast_days={}",
            51.68, 5.05, 1        
        );

        let res = reqwasm::http::Request::get(&url)
        .send()
        .await
        .map_err(|_| FetchError::Request)?
        .json::<WeatherForecast>()
        .await
        .map_err(|_| FetchError::Request)?;

        Ok(res.daily.temperature_2m_max)
    } else {
        Err(FetchError::NonZeroDays)
    }
}

pub fn fetch_weather(cx: Scope) -> impl IntoView {
    let (days, _set_days) = create_signal::<WeatherForecastDays>(cx, 1);

    let weather_forecast = create_local_resource(cx, days, fetch_weather_forecast);

    let fallback = move |cx, errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { cx, <li>{e.to_string()}</li> })
                    .collect_view(cx)
            })
        };

        view! { cx,
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let weather_forecast_view = move || {
        weather_forecast.read(cx).map(|data| {
            data.map(|data| {
                data.iter()
                    .map(|s| {
                        let temperature = *s;
                        let polo_can_go_out = temperature > 27.00;
                        view! { 
                            cx, 
                            <div class="polo-card">
                                <div>
                                <h1>"Kan de Polo uit?"</h1>
                                <p>"De maximum temperatuur van vandaag is "{temperature.to_string()}"°C"</p>
                                <p class=(if polo_can_go_out { "yeet" } else { "keep" })>
                                   {
                                        if polo_can_go_out {
                                            "Ja"
                                        } else {
                                            "Nee"
                                        }
                                    }
                                </p>
                                </div>
                            </div>
                        }
                    })
                    .collect_view(cx)
            })
        })
    };

    view! { cx,
        <div>
            <ErrorBoundary fallback>
                <Transition fallback=move || {
                    view! { cx, <div>"Loading (Suspense Fallback)..."</div> }
                }>
                <div>
                    {weather_forecast_view}
                </div>
                </Transition>
            </ErrorBoundary>
        </div>
    }
}

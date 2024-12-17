use yew::prelude::*;
use crate::recommendation::Movie;
use crate::components::range_slider::RangeSlider;

#[derive(Clone, PartialEq)]
pub enum FloatingWindowMode {
    SelectMovies,
    Preferences,
}

#[derive(Properties, Clone, PartialEq)]
pub struct FloatingWindowProps {
    pub mode: FloatingWindowMode,
    pub movies: Vec<Movie>,                    
    pub on_movie_select: Callback<Movie>,     
    pub movie_count: usize, 
    pub callback_button: Callback<()>,         
    pub year_range: (i32, i32),                
    pub runtime_range: (i32, i32),             
    pub rating: f32,                           
    pub on_year_update: Callback<(i32, i32)>,  
    pub on_runtime_update: Callback<(i32, i32)>, 
    pub on_rating_update: Callback<f32>,       
}

#[function_component(FloatingWindow)]
pub fn floating_window(props: &FloatingWindowProps) -> Html {
    match props.mode {
        FloatingWindowMode::SelectMovies => html! {
            <div class="floating-window">
                <h2 style="text-align: center;">{ "Select Movies You Like" }</h2>
                
                <div class="movie-container horizontal">
                    {
                        for props.movies.iter().map(|movie| {
                            let movie_clone = movie.clone();
                            let title = movie.title.clone().unwrap_or("No Title".to_string());
                            let image_url = movie.image_url.clone().unwrap_or_default();

                            html! {
                                <div
                                    class="movie"
                                    onclick={props.on_movie_select.reform(move |_| movie_clone.clone())}
                                >
                                    <img src={image_url} alt={title.clone()} />
                                    <p>{ title }</p>
                                </div>
                            }
                        })
                    }
                </div>

                <div style="text-align: center; margin-bottom: 15px;">
                    <div style="width: 100%; background-color: #eee; border-radius: 15px; overflow: hidden; height: 10px;">
                        <div style={format!(
                            "width: {}%; background-color: #4caf50; height: 100%;",
                            (props.movie_count as f32 / 10.0) * 100.0
                        )}></div>
                    </div>
                </div>
                <div style="text-align: center;">
                    <button onclick={props.callback_button.reform(|_| ())} class="built-in-button">{ "Refresh" }</button>
                </div>
            </div>
        },
        FloatingWindowMode::Preferences => html! {
            <div class="floating-window">
                <h2 style="text-align: center;">{ "Set Preferences" }</h2>
                <div class="sliders">
                    <div class="slider-container">
                        <label>{ format!("Year Range: {} - {}", props.year_range.0, props.year_range.1) }</label>
                        <RangeSlider
                            min=1900.0
                            max=2023.0
                            step=1.0
                            start={(props.year_range.0 as f64, props.year_range.1 as f64)}
                            on_update={Callback::from({
                                let on_year_update = props.on_year_update.clone();
                                move |(min, max): (f64, f64)| {
                                    on_year_update.emit((min as i32, max as i32));
                                }
                            })}
                        />
                    </div>
                    <div class="slider-container">
                        <label>{ format!("Runtime Range: {} mins - {} mins", props.runtime_range.0, props.runtime_range.1) }</label>
                        <RangeSlider
                            min=30.0
                            max=300.0
                            step=1.0
                            start={(props.runtime_range.0 as f64, props.runtime_range.1 as f64)}
                            on_update={Callback::from({
                                let on_runtime_update = props.on_runtime_update.clone();
                                move |(min, max): (f64, f64)| {
                                    on_runtime_update.emit((min as i32, max as i32));
                                }
                            })}
                        />
                    </div>
                    <div class="slider-container">
                        <label>{ format!("Rating > {:.1}", props.rating) }</label>
                        <input
                            type="range"
                            min="0"
                            max="10"
                            step="0.1"
                            value={props.rating.to_string()}
                            oninput={props.on_rating_update.reform(|e: InputEvent| {
                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                input.value().parse::<f32>().unwrap_or(0.0)
                            })}
                        />
                    </div>
                    <div style="text-align: center;">
                        <button onclick={props.callback_button.reform(|_| ())} class="built-in-button">{ "Done" }</button>
                    </div>
                </div>
            </div>
        },
    }
}

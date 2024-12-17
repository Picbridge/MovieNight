use yew::prelude::*;
use yew::platform::spawn_local;
use wasm_bindgen::JsCast;
use serde_json::json;
use crate::JsValue;
use crate::recommendation::Movie;
use crate::config::api_base;
use crate::components::md_viewer::MarkdownViewer;

#[derive(Properties, Clone, PartialEq)]
pub struct MovieListProps {
    pub selected_movies: Option<Vec<Movie>>,
    pub movies: Vec<Movie>,
    pub on_movie_highlight: Callback<Movie>,
}

pub struct MovieList {
    reasoning: String,
    request_sent: bool,
}

pub enum Msg {
    FetchReasoning,
    UpdateReasoning(String),
}

impl Component for MovieList {
    type Message = Msg;
    type Properties = MovieListProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchReasoning);
        
        Self {
            reasoning: "Fetching reasoning...".to_string(),
            request_sent: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchReasoning => {
                let props = ctx.props();
                let movies = &props.movies;
                let selected_movies = &props.selected_movies;
                if let Some(selected_movies) =  selected_movies {
                    if !selected_movies.is_empty() && !self.request_sent {
                        let url = format!("{}/recommender/reasoning", api_base());
        
                        // Clone the data for async move
                        let selected_movie_names: Vec<String> = selected_movies
                            .iter()
                            .map(|movie| movie.title.clone().unwrap_or_default())
                            .collect();
        
                        let recommended_movie_names: Vec<String> = movies
                            .iter()
                            .map(|movie| movie.title.clone().unwrap_or_default())
                            .collect();
                        
                        let link = ctx.link().clone();
                        spawn_local(async move {
                            let payload = json!({
                                "selected_movie": selected_movie_names,
                                "recommended_movie": recommended_movie_names,
                            });
                        
                            let opts = web_sys::RequestInit::new();
                            opts.set_method("POST");
                            opts.set_mode(web_sys::RequestMode::Cors);
                            opts.set_credentials(web_sys::RequestCredentials::Include);
                            opts.set_body(&JsValue::from_str(&payload.to_string())); // Fix here
                        
                            let headers = web_sys::Headers::new().unwrap();
                            headers.set("Content-Type", "application/json").unwrap();
                            opts.set_headers(headers.dyn_into::<JsValue>().unwrap().as_ref()); // Fix here
                        
                            let window = web_sys::window().expect("No global `window` exists");
                            let response = wasm_bindgen_futures::JsFuture::from(
                                window.fetch_with_str_and_init(&url, &opts),
                            )
                            .await
                            .and_then(|resp| resp.dyn_into::<web_sys::Response>())
                            .expect("Failed to fetch");
                        
                            if response.ok() {
                                if let Ok(js_value) = wasm_bindgen_futures::JsFuture::from(response.text().unwrap()).await {
                                    let response_text = js_value.as_string().unwrap_or_else(|| {
                                        log::error!("Failed to convert response to string");
                                        "Failed to fetch reasoning.".to_string()
                                    });
                            
                                    //log::info!("Raw response text: {}", response_text);
                            
                                    // Parse the outer stringified JSON
                                    match serde_json::from_str::<serde_json::Value>(&response_text) {
                                        Ok(outer_json) => {
                                            //log::info!("Outer JSON parsed: {:?}", outer_json);
                            
                                            // Parse the inner JSON
                                            if let Some(inner_json_str) = outer_json.as_str() {
                                                match serde_json::from_str::<serde_json::Value>(inner_json_str) {
                                                    Ok(inner_json) => {
                                                        //log::info!("Inner JSON parsed: {:?}", inner_json);
                            
                                                        if let Some(reason) = inner_json.get("reason").and_then(|r| r.as_str()) {
                                                            link.send_message(Msg::UpdateReasoning(reason.to_string()));
                                                        } else {
                                                            log::error!("`reason` field not found in inner JSON.");
                                                        }
                                                    }
                                                    Err(err) => {
                                                        log::error!("Failed to parse inner JSON: {}", err);
                                                    }
                                                }
                                            } else {
                                                log::error!("Outer JSON does not contain a valid inner JSON string.");
                                            }
                                        }
                                        Err(err) => {
                                            log::error!("Failed to parse outer JSON: {}", err);
                                        }
                                    }
                                }
                            }
                            
                        });

                        self.request_sent = true;
                    }
                }
                false
            }
            Msg::UpdateReasoning(new_reasoning) => {
                self.reasoning = new_reasoning;
                //log::info!("Extracted reasoning: {}", self.reasoning.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <>
                <div class="movie-container horizontal">
                    { for props.movies.iter().map(|movie| {
                        let movie_clone = movie.clone();
                        let title = movie.title.clone().unwrap_or("No Title".to_string());
                        let image_url = movie.image_url.clone().unwrap_or_default();

                        html! {
                            <div
                                class="movie"
                                onclick={props.on_movie_highlight.reform(move |_| movie_clone.clone())}
                            >
                                <img src={image_url} alt={title.clone()} />
                                <p>{ title }</p>
                            </div>
                        }
                    }) }
                </div>

                <div>
                    { if let Some(_selected_movies) = &props.selected_movies {
                        html! {
                            <div>
                                <p>{"Reasoning: "}</p>
                                <div class="reasoning">
                                   {self.reasoning.clone() }
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </>
        }
    }
}


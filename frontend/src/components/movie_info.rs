use yew::prelude::*;
use yew::platform::spawn_local;
//use gloo_net::http::Request; //Use for like
use wasm_bindgen::JsCast;
use crate::JsValue;
use crate::config::api_base;
use crate::recommendation::Movie;

/// Define messages for the component.
pub enum Msg {
    ClosePopup,
    FetchWhereToWatch,
    UpdateWhereToWatch(String),
}

/// Define the properties for the component.
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub image_url: String,
    pub title: String,
    pub released_year: String,
    pub stars: String,
    pub description: String,
    pub director: String,
    pub rating: String,
    pub genres: Vec<String>,
    pub on_close: Callback<()>,
}

impl Props {
    pub fn from_movie(movie: &Movie, on_close: Callback<()>) -> Props {
        Props {
            image_url: movie.image_url.clone().unwrap_or_default(),
            title: movie.title.clone().unwrap_or("No Title".to_string()),
            released_year: movie.released_year.clone().unwrap_or("N/A".to_string()),
            stars: movie.stars.clone().unwrap_or_default().join(", "),
            description: movie.description.clone().unwrap_or("No Description".to_string()),
            director: movie.director.clone().unwrap_or("No Director Info".to_string()),
            rating: movie.imdb_rating.clone().unwrap_or("N/A".to_string()),
            genres: movie.genres.clone().unwrap_or_default(),
            on_close,
        }
    }
}

/// Define the component struct.
pub struct MovieInfo {
    where_to_watch: Option<String>, // State to hold the "Where to Watch" information
}

impl Component for MovieInfo {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchWhereToWatch);
        MovieInfo {
            where_to_watch: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClosePopup => {
                ctx.props().on_close.emit(());
                false
            }
            Msg::FetchWhereToWatch => {
                let url = format!("{}/recommender/where", api_base());
                let link = ctx.link().clone();
                let movie_title = ctx.props().title.clone();
                log::info!("Sending POST request to {}", url);
                
                spawn_local(async move {
                    let payload = serde_json::json!({
                        "title": movie_title,
                    });
    
                    // Initialize request options
                    let opts = web_sys::RequestInit::new();
                    opts.set_method("POST");
                    opts.set_credentials(web_sys::RequestCredentials::Include);
                    opts.set_mode(web_sys::RequestMode::Cors);
                    opts.set_body(&JsValue::from_str(&payload.to_string()));
            
                    // Set the Content-Type header
                    let request_headers = web_sys::Headers::new().unwrap();
                    request_headers.set("Content-Type", "application/json").unwrap();
                    opts.set_headers(&request_headers);
            
                    let window = web_sys::window().expect("No global `window` exists");
                    let response = match wasm_bindgen_futures::JsFuture::from(window.fetch_with_str_and_init(&url, &opts)).await {
                        Ok(resp) => resp.dyn_into::<web_sys::Response>().unwrap(),
                        Err(err) => {
                            log::error!("Failed to fetch: {:?}", err);
                            return;
                        }
                    };
            
                    if response.ok() {
                        match wasm_bindgen_futures::JsFuture::from(response.text().unwrap()).await {
                            Ok(js_value) => {
                                let otts = js_value.as_string().unwrap_or_else(|| "Failed to fetch".to_string());
                                log::info!("User is valid: {}", otts);
                                link.send_message(Msg::UpdateWhereToWatch(otts));
                            }
                            Err(_) => {
                                log::error!("Failed to parse plain text response.");
                            }
                        }
                    } else {
                        log::error!("Error: {} - {}", response.status(), response.status_text());
                    }
                });
                
                false
            }
            Msg::UpdateWhereToWatch(data) => {
                self.where_to_watch = Some(data);
                true // Re-render the component
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            image_url,
            title,
            released_year,
            stars,
            description,
            director,
            rating,
            genres,
            ..
        } = &ctx.props();
    
        let genres_text = genres.join(", ");
        let where_to_watch = self
            .where_to_watch
            .clone();
    
        html! {
            <div class="popup-overlay">
                <div class="popup">
                    <button class="close-button" onclick={ctx.link().callback(|_| Msg::ClosePopup)}>{"×"}</button>
                    <div class="popup-content">
                        <div class="popup-left">
                            <img src={image_url.clone()} alt={title.clone()} />
                        </div>
                        <div class="popup-right">
                            <h2>{ title }</h2>
                            <p><strong>{"Starring: "}</strong>{ stars }</p>
                            <p><strong>{"Director: "}</strong>{ director }</p>
                            <p><strong>{"Genres: "}</strong>{ genres_text }</p>
                            <p><strong>{"IMDb Rating: "}</strong>{ rating }</p>
                            <p><strong>{"Released: "}</strong>{ released_year }</p>
                            <p><strong>{"Where to Watch: "}</strong></p>
                            {
                                if let Some(platforms) = where_to_watch {
                                    html! { <p>{ platforms }</p> }
                                } else {
                                    html! {
                                        <div class="spinner-container">
                                            <div class="spinner"></div>
                                            <p>{ "Asking Gemini..." }</p>
                                        </div>
                                    }
                                }
                            }
                        </div>
                    </div>
                    <div class="popup-description">
                        <p><strong>{ "Description:" }</strong></p>
                        <p>{ description }</p>
                    </div>
                </div>
            </div>
        }
    }    
}

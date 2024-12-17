use yew::prelude::*;
use yew_router::prelude::*;
use yew::platform::spawn_local;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::app::AppRoute;
use crate::config::*;
use crate::auth_context::SharedAuthContext;
use crate::components::movie_info::{MovieInfo, Props as MovieInfoProps};
use crate::components::floating_window::{FloatingWindow, FloatingWindowMode};
use crate::components::movie_list::MovieList;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Movie {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "imdb_rating")]
    pub imdb_rating: Option<String>,
    pub stars: Option<Vec<String>>,
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    #[serde(rename = "released_year")]
    pub released_year: Option<String>,
    pub runtime: Option<String>,
    pub metadata: Option<String>,
    pub director: Option<String>,
    pub genres: Option<Vec<String>>,
}

pub struct Recommendation {
    auth_context: SharedAuthContext,
    _context_handle: ContextHandle<SharedAuthContext>,
    message: String,
    random_movies: Vec<Movie>,     
    selected_movies: Vec<Movie>,   
    movie_count: usize,            
    show_window: bool,         
    year_range: (i32, i32),        // Start and end year
    runtime_range: (i32, i32),     // Min and max runtime in minutes
    rating: f32,     
    recommended_movies: Vec<Movie>,
    movie_highlight: Option<Movie>,
}

pub enum Msg {
    GetRandomMovies(usize),        
    ReceiveRandomMovie(Movie),     
    ClearRandomMovies,             
    SelectMovie(Movie),
    NextMovies,
    UpdateYearRange((i32, i32)),
    UpdateRuntimeRange((i32, i32)),
    UpdateRating(f32),
    UpdateRecommendations(Vec<Movie>),
    PreferencesDone,
    Error(String),
    HighlightMovie(Movie),
    CloseHighlight,
    ContextChanged(SharedAuthContext),
}

impl Component for Recommendation {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (auth_context, context_handle) = ctx
            .link()
            .context::<SharedAuthContext>(ctx.link().callback(Msg::ContextChanged))
            .expect("No AuthContext provided");
        

        if !auth_context.borrow().is_logged_in {
            let navigator = ctx.link().navigator().unwrap(); // Get the navigator instance
            navigator.push(&AppRoute::Login);
        }
        let recommendation = Recommendation {
            auth_context,
            _context_handle: context_handle,
            message: String::new(),
            random_movies: Vec::new(),
            selected_movies: Vec::new(),
            movie_count: 0,
            show_window: true,
            year_range: (2000, 2023),       
            runtime_range: (60, 180),     
            rating: 5.0, 
            recommended_movies: Vec::new(),
            movie_highlight: None,
        };

        // Fetch 5 random movies when the component is created
        ctx.link().send_message(Msg::GetRandomMovies(5));

        recommendation
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetRandomMovies(count) => {
                let url = format!("{}/recommender/random", api_base());

                let link = ctx.link().clone();

                for _ in 0..count {
                    let url_clone = url.clone();
                    let link_clone = link.clone();

                    spawn_local(async move {
                        let response = Request::get(&url_clone)
                            .header("Content-Type", "application/json")
                            .send()
                            .await;

                        match response {
                            Ok(resp) if resp.status() == 200 => {
                                let response_json: Result<Movie, _> = resp.json().await;
                                match response_json {
                                    Ok(movie) => {
                                        link_clone.send_message(Msg::ReceiveRandomMovie(movie));
                                    }
                                    Err(e) => {
                                        log::error!("Failed to parse JSON: {:?}", e);
                                        link_clone.send_message(Msg::Error("Failed to parse movie data".into()));
                                    }
                                }
                            }
                            Ok(resp) => {
                                log::error!("Failed to fetch movie: {}", resp.status());
                                link_clone.send_message(Msg::Error("Failed to fetch movie".into()));
                            }
                            Err(e) => {
                                log::error!("Request error: {:?}", e);
                                link_clone.send_message(Msg::Error("Request error".into()));
                            }
                        }
                    });
                }
                false
            }

            Msg::ReceiveRandomMovie(movie) => {
                // Check if the movie is already selected or in random_movies to avoid duplicates
                let movie_id = movie.id.clone();
                let is_duplicate = self.selected_movies.iter().any(|m| m.id == movie_id)
                    || self.random_movies.iter().any(|m| m.id == movie_id);

                if !is_duplicate {
                    self.random_movies.push(movie);
                    true
                } else {
                    // If duplicate, fetch another movie
                    ctx.link().send_message(Msg::GetRandomMovies(1));
                    false
                }
            }

            Msg::ClearRandomMovies => {
                self.random_movies.clear();
                true
            }

            Msg::SelectMovie(movie) => {
                if self.movie_count < 10 {
                    self.selected_movies.push(movie.clone());
                    self.movie_count += 1;

                    // Remove the selected movie from random_movies to prevent re-selection
                    self.random_movies.retain(|m| m.id != movie.id);

                    // if self.movie_count == 10 {
                    //     self.show_window = false; // Close the window after 10 movies are selected
                    // } else 
                    if self.random_movies.len() < 5 {
                        // Fetch additional movies to maintain 5 movies in the window
                        let movies_needed = 5 - self.random_movies.len();
                        ctx.link().send_message(Msg::GetRandomMovies(movies_needed));
                    }
                }
                true
            }

            Msg::NextMovies => {
                // Clear the current random_movies and fetch 5 new movies
                ctx.link().send_message(Msg::ClearRandomMovies);
                ctx.link().send_message(Msg::GetRandomMovies(5));
                false
            }

            Msg::Error(error_message) => {
                self.message = error_message;
                true
            }

            Msg::UpdateRating(rating) => {
                self.rating = rating;
                true
            }
            Msg::UpdateYearRange(range) => {
                self.year_range = range;
                true
            }
            Msg::UpdateRuntimeRange(range) => {
                self.runtime_range = range;
                true
            }
            Msg::UpdateRecommendations(movies) => {
                let auth_context = self.auth_context.clone();
                self.recommended_movies = movies.clone();
                log::info!("is_logged_in: {}", auth_context.borrow().is_logged_in);

                let user_id = auth_context.borrow().user_id.clone().unwrap_or_else(|| "None".to_string());
                log::info!("user_id: {}", user_id);
                if auth_context.borrow().is_logged_in {
                    // Post movie recommendations to the container
                    let url = format!("{}/recommendation/push", api_base());
                    
                    let link = ctx.link().clone();
                    log::info!("Sending POST request to {}", url);
                    spawn_local(async move {
                        let payload = serde_json::json!({
                            "user_id": auth_context.borrow().user_id,
                            "movies": movies.clone(),
                        });
                        
                        //log::info!("Payload: {:?}", payload);
                        let request_builder = Request::post(&url)
                            .header("Content-Type", "application/json")
                            .body(payload.to_string());

                        match request_builder {
                            Ok(request) => {
                                let response = request.send().await;

                                match response {
                                    Ok(resp) if resp.ok() => {
                                        let json: Value = resp.json().await.unwrap_or(Value::Null);
                                        //log::info!("Recommendation response: {:?}", json);
                                    }
                                    Ok(resp) => {
                                        link.send_message(Msg::Error(format!("Error: {}", resp.status())));
                                    }
                                    Err(err) => {
                                        link.send_message(Msg::Error(format!("Failed to fetch: {}", err)));
                                    }
                                }
                            }
                            Err(err) => {
                                link.send_message(Msg::Error(format!("Failed to build request: {}", err)));
                            }
                        }
                    });
                }
                
                true
            }
            Msg::PreferencesDone => {
                self.show_window = false;

                // Send the selected movies and preferences to the server 
                let url = format!("{}/recommender/recommend", api_base());

                let link = ctx.link().clone();
                let selected_movies = self.selected_movies.clone();
                let year_range = self.year_range;
                let runtime_range = self.runtime_range;
                let rating = self.rating;
                log::info!("Sending POST request to {}", url);
                
                spawn_local(async move {
                    let payload = serde_json::json!({
                        "movies": selected_movies,
                        "year_range": year_range,
                        "runtime_range": runtime_range,
                        "rating": rating,
                    });

                    let request_builder = Request::post(&url)
                        .header("Content-Type", "application/json")
                        .body(payload.to_string());

                        match request_builder {
                            Ok(request) => {
                                let response = request.send().await;
    
                                match response {
                                    Ok(resp) if resp.ok() => {
                                        let movies: Vec<Movie> = match resp.json().await {
                                            Ok(data) => data,
                                            Err(err) => {
                                                log::error!("Failed to deserialize movies: {}", err);
                                                vec![]  // or handle the error as needed
                                            }
                                        };
                                
                                        link.send_message(Msg::UpdateRecommendations(movies));
                                    }
                                    Ok(resp) => {
                                        link.send_message(Msg::Error(format!("Error: {}", resp.status())));
                                    }
                                    Err(err) => {
                                        link.send_message(Msg::Error(format!("Failed to fetch: {}", err)));
                                    }
                                }
                                
                            }
                            Err(err) => {
                                link.send_message(Msg::Error(format!("Failed to build request: {}", err)));
                            }
                        }
                    });
                true // Return true to re-render the component
            }

            Msg::HighlightMovie(movie) => {
                self.movie_highlight = Some(movie);
                true
            }

            Msg::CloseHighlight => {
                self.movie_highlight = None;
                true
            }

            Msg::ContextChanged(new_context) => {
                self.auth_context = new_context;
                true 
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let floating_window = if self.show_window && self.movie_count < 10 {
            html! {
                <FloatingWindow
                    mode={FloatingWindowMode::SelectMovies}
                    movies={self.random_movies.clone()}
                    on_movie_select={ctx.link().callback(Msg::SelectMovie)}
                    movie_count={self.movie_count}
                    callback_button={ctx.link().callback(|_| Msg::NextMovies)}
                    year_range={self.year_range}
                    runtime_range={self.runtime_range}
                    rating={self.rating}
                    on_year_update={Callback::noop()} // Not used in this mode
                    on_runtime_update={Callback::noop()} // Not used in this mode
                    on_rating_update={Callback::noop()} // Not used in this mode
                />
            }
        } else if self.show_window && self.movie_count == 10 {
            html! {
                <FloatingWindow
                    mode={FloatingWindowMode::Preferences}
                    movies={vec![]} // Not used in this mode
                    on_movie_select={Callback::noop()} // Not used in this mode
                    movie_count={self.movie_count}
                    callback_button={ctx.link().callback(|_| Msg::PreferencesDone)}
                    year_range={self.year_range}
                    runtime_range={self.runtime_range}
                    rating={self.rating}
                    on_year_update={ctx.link().callback(Msg::UpdateYearRange)}
                    on_runtime_update={ctx.link().callback(Msg::UpdateRuntimeRange)}
                    on_rating_update={ctx.link().callback(Msg::UpdateRating)}
                />
            }
        } else {
            html! { <></> }
        };

        html! {
            <div class="page-background">
                <div class="logo-container">
                    <Link<AppRoute> to={AppRoute::Home}>
                        <img src={logo_src()} alt="Logo" class="logo" />
                    </Link<AppRoute>>
                    <div class="logo-divider"></div> // Divider line
                </div>
                <h1>{ "Recommendation" }</h1>
        
                { floating_window }
        
                // Render MovieList and MovieInfo only when show_window is false and movie_count == 10
                {
                    if !self.show_window && self.movie_count == 10 && self.recommended_movies.clone().len() == 5 {
                        html! {
                            <>
                                <MovieList 
                                    selected_movies={self.selected_movies.clone()}
                                    movies={self.recommended_movies.clone()} 
                                    on_movie_highlight={ctx.link().callback(Msg::HighlightMovie)} 
                                />
        
                                {
                                    if let Some(selected_movie) = &self.movie_highlight {
                                        html! {
                                            <MovieInfo
                                                ..MovieInfoProps::from_movie(
                                                    selected_movie,
                                                    ctx.link().callback(|_| Msg::CloseHighlight),
                                                )
                                            />
                                        }
                                    } else {
                                        html! { <></> }
                                    }
                                }
                            </>
                        }
                    } else {
                        html! { <></> }
                    }
                }
            </div>
        }
    }
    
}
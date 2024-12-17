use yew::prelude::*;
use yew::platform::spawn_local;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::config::api_base;
use crate::recommendation::Movie;
use crate::components::movie_info::{MovieInfo, Props as MovieInfoProps};
use crate::components::movie_list::MovieList;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Recommendation {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub movies: Option<Vec<Movie>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub user: String,
}

pub enum Msg {
    FetchHistory,
    ReceiveHistory(Vec<Recommendation>),
    SelectMovie(Movie),
    ClosePopup,
}

pub struct History {
    history: Vec<Recommendation>,
    selected_movie: Option<Movie>,
}

impl Component for History {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(Msg::FetchHistory);

        History {
            history: vec![],
            selected_movie: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchHistory => {
                let url = format!("{}/recommendation/pull", api_base());
                let user = ctx.props().user.clone();

                log::info!("Sending GET request to {}", url);

                let link = ctx.link().clone();
                spawn_local(async move {
                    let payload = serde_json::json!({
                        "user_id": user,
                        "movies": Vec::<Movie>::new()
                    });

                    let request_builder = Request::post(&url)
                        .header("Content-Type", "application/json")
                        .body(payload.to_string());

                    match request_builder {
                        Ok(request) => {
                            let response = request.send().await;
    
                             match response {
                                Ok(resp) if resp.ok() => {
                                    let history: Vec<Recommendation> = match resp.json().await {
                                        Ok(data) => data,
                                        Err(err) => {
                                            log::error!("Failed to parse history: {:?}", err);
                                            vec![]
                                        }
                                    };

                                    link.send_message(Msg::ReceiveHistory(history));
                                }
                                Ok(resp) => {
                                    log::error!("Failed to fetch history: {:?}", resp);
                                }
                                Err(err) => {
                                    log::error!("Failed to fetch history: {:?}", err);
                                }
                            }
                        }
                        Err(err) => {
                            log::error!("Failed to build request: {:?}", err);
                        }
                    }
                });

                false
            }
            Msg::ReceiveHistory(history) => {
                self.history = history;
                true
            }
            Msg::SelectMovie(movie) => {
                self.selected_movie = Some(movie);
                true
            }
            Msg::ClosePopup => {
                self.selected_movie = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="history">
                <h1>{ "Recommendation History" }</h1>
                <div class="history-container">
                    { for self.history.iter().rev().map(|recommendation| {
                            let created_at = recommendation
                                .created_at
                                .map(|dt| dt.format("%Y-%m-%d").to_string())
                                .unwrap_or("Unknown date".to_string());

                                html! {
                                    <div class="recommendation">
                                        <p>{ created_at }</p>           
                                        { 
                                            html! {
                                                <>
                                                    <MovieList
                                                        movies={recommendation.movies.clone().unwrap_or_default()}
                                                        on_movie_highlight={ctx.link().callback(Msg::SelectMovie)}
                                                    />
                                                    {
                                                        if let Some(selected_movie) = &self.selected_movie {
                                                            html! {
                                                                <MovieInfo
                                                                    ..MovieInfoProps::from_movie(
                                                                        selected_movie,
                                                                        ctx.link().callback(|_| Msg::ClosePopup),
                                                                    )
                                                                />
                                                            }
                                                        } else {
                                                            html! { <></> } // Fallback for no highlighted movie
                                                        }
                                                    }
                                                </>
                                            }
                                        }
                                        <div class="logo-divider"></div>
                                    </div>
                                }
                        }) 
                    }
                </div>
            </div>
        }
    }
    
}

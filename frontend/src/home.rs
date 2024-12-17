use yew::prelude::*;
use yew_router::prelude::*;
use yew::platform::spawn_local;
use wasm_bindgen::JsCast;
use crate::auth_context::{SharedAuthContext};
use crate::app::AppRoute;
use crate::config::*;
use crate::components::sidebar::Sidebar;
use crate::recommendation::Movie;
use crate::components::movie_info::{MovieInfo, Props as MovieInfoProps};
use crate::components::history::History;

pub struct Home {
    auth_context: SharedAuthContext,
    movie_highlight: Option<Movie>,
    show_history: bool,
}

pub enum Msg {
    CheckUser, // Triggered to check user login state
    UserValid(String), // Triggered when user is valid
    GetRandomMovie,
    HighlightMovie(Movie),
    CloseHighlight,
    ToggleHistory,
    Logout, // Triggered when user is invalid
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();
 
    fn create(ctx: &Context<Self>) -> Self {
        //dotenv().ok();
        
        let (auth_context, _context_handle) = ctx
        .link()
        .context::<SharedAuthContext>(Callback::noop())
        .expect("No AuthContext provided");
            
        ctx.link().send_message(Msg::CheckUser);

        Self {
            auth_context,
            movie_highlight: None,
            show_history: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CheckUser => {
                let url = format!("{}/user/isvalid", api_base());
            
                log::info!("Checking if user is valid: {}", url);
            
                let link = ctx.link().clone();
                spawn_local(async move {
                    // Initialize request options
                    let opts = web_sys::RequestInit::new();
                    opts.set_method("GET");
                    opts.set_credentials(web_sys::RequestCredentials::Include);
                    opts.set_mode(web_sys::RequestMode::Cors);
            
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
                                let user = js_value.as_string().unwrap_or_else(|| "Unknown User".to_string());
                                log::info!("User is valid: {}", user);
                                link.send_message(Msg::UserValid(user));
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
            // Update the user state
            Msg::UserValid(user) => {
                self.auth_context.borrow_mut().is_logged_in = true;
                self.auth_context.borrow_mut().user_id = Some(user);
                true
            }
            Msg::Logout => {
                let url = format!("{}/user/logout", api_base());
            
                log::info!("Logging out: {}", url);
                spawn_local(async move {
                    // Initialize request options
                    let opts = web_sys::RequestInit::new();
                    opts.set_method("POST");
                    opts.set_credentials(web_sys::RequestCredentials::Include);
                    opts.set_mode(web_sys::RequestMode::Cors);
            
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
                            Ok(_js_value) => {
                                log::info!("User is logged out.");
                            }
                            Err(_) => {
                                log::error!("Failed to parse plain text response.");
                            }
                        }
                    } else {
                        log::error!("Error: {} - {}", response.status(), response.status_text());
                    }
                });

                self.auth_context.borrow_mut().is_logged_in = false;
                self.auth_context.borrow_mut().user_id = None;
                true
            }

            Msg::GetRandomMovie => {
                let url = format!("{}/recommender/random", api_base());
                let link = ctx.link().clone();
                log::info!("Fetching random movie: {}", url);
            
                spawn_local(async move {
                    // Configure the request options
                    let mut opts = web_sys::RequestInit::new();
                    opts.method("GET");
                    opts.credentials(web_sys::RequestCredentials::Include);
                    opts.mode(web_sys::RequestMode::Cors);
            
                    // Create the fetch request
                    let request = web_sys::Request::new_with_str_and_init(&url, &opts).expect("Failed to create request");
            
                    // Use the browser's fetch API
                    let window = web_sys::window().expect("No global `window` exists");
                    let fetch_response = match wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await {
                        Ok(resp) => resp.dyn_into::<web_sys::Response>().unwrap(),
                        Err(err) => {
                            log::error!("Failed to fetch: {:?}", err);
                            return;
                        }
                    };
            
                    // Handle the response
                    if fetch_response.ok() {
                        match wasm_bindgen_futures::JsFuture::from(fetch_response.json().unwrap()).await {
                            Ok(json) => {
                                if let Ok(movie) = json.into_serde::<Movie>() {
                                    link.send_message(Msg::HighlightMovie(movie));
                                } else {
                                    log::error!("Failed to parse JSON into Movie.");
                                }
                            }
                            Err(err) => {
                                log::error!("Failed to parse response JSON: {:?}", err);
                            }
                        }
                    } else {
                        log::error!("Request failed with status: {}", fetch_response.status());
                    }
                });
            
                false
            }            
            
            Msg::HighlightMovie(movie) => {
                self.movie_highlight = Some(movie);
                true
            }
            Msg::CloseHighlight => {
                self.movie_highlight = None;
                true
            }

            Msg::ToggleHistory => {
                self.show_history = !self.show_history; // Toggle visibility
                true
            }
        } 
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let auth_context = self.auth_context.borrow();
    
        let sidebar_contents = html! {
            <div style="display: flex; flex-direction: column;">
                <Link<AppRoute> to={AppRoute::Recommendation}>
                    <button class="sidebar-content-button">{ "Get Recommendation" }</button>
                </Link<AppRoute>>
                <button onclick={ctx.link().callback(|_| Msg::GetRandomMovie)} class="sidebar-content-button">
                    { "Random Movie" }
                </button>
                <button onclick={ctx.link().callback(|_| Msg::ToggleHistory)} class="sidebar-content-button">
                    { "History" }
                </button>
                <button onclick={ctx.link().callback(|_| Msg::Logout)} class="sidebar-content-button">
                    { "Logout" }
                </button>
            </div>
        };
    
        html! {
            <div class="page-background">
                <div class="logo-container">
                    <Link<AppRoute> to={AppRoute::Home}>
                        <img src={logo_src()} alt="Logo" class="logo" />
                    </Link<AppRoute>>
                    <div class="logo-divider"></div>
                </div>
    
                <div class="auth-container">
                    {
                        if auth_context.is_logged_in {
                            html! {
                                <div class="auth-buttons">
                                    <Sidebar>
                                        { sidebar_contents }
                                    </Sidebar>
                                </div>
                            }
                        } else {
                            html! {
                                <div class="auth-buttons">
                                    <Link<AppRoute> to={AppRoute::Login}>
                                        <button class="built-in-button">{ "Login" }</button>
                                    </Link<AppRoute>>
                                    <Link<AppRoute> to={AppRoute::Register}>
                                        <button class="built-in-button">{ "Signup" }</button>
                                    </Link<AppRoute>>
                                </div>
                            }
                        }
                    }
                </div>
                {
                    if let Some(movie) = &self.movie_highlight {
                        html! {
                            <MovieInfo
                                ..MovieInfoProps::from_movie(
                                    movie,
                                    ctx.link().callback(|_| Msg::CloseHighlight),
                                )
                            />
                        }
                    } else {
                        html! { <></> } // Fallback for no highlighted movie
                    }
                }
                {
                    if self.show_history {
                        html! {
                            <History user={auth_context.user_id.clone().unwrap_or_default()} />
                        }
                    } else {
                        html! {
                            <main class="main-content">
                            {
                                if auth_context.is_logged_in {
                                    html! {
                                        <Link<AppRoute> to={AppRoute::Recommendation}>
                                            <button class="built-in-button">{ "Get Recommendation" }</button>
                                        </Link<AppRoute>>
                                    }
                                } else {
                                    html! {
                                        <Link<AppRoute> to={AppRoute::Login}>
                                            <button class="built-in-button">{ "Get Recommendation" }</button>
                                        </Link<AppRoute>>
                                    }
                                }
                            }
                            </main>
                        }
                    }
                }
    
                
            </div>
        }
    }
    
}
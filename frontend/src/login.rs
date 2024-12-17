use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::AppRoute;
use yew::platform::spawn_local;
use wasm_bindgen::JsCast;
use crate::JsValue;
use crate::config::*;

pub struct Login {
    id: String,
    password: String,
    show_modal: bool, // Add a modal state
}

pub enum Msg {
    UpdateId(String),
    UpdatePassword(String),
    Submit,
    ReceiveResponse(Result<String, String>),
    CloseModal, // Add a close modal message
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Login {
            id: String::new(),
            password: String::new(),
            show_modal: false, // Initialize the modal state
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateId(value) => {
                self.id = value;
                false
            }
            Msg::UpdatePassword(value) => {
                self.password = value;
                false
            }
            Msg::Submit => {
                let id = self.id.clone();
                let password = self.password.clone();
                let url = format!("{}/user/login", api_base());
            
                log::info!("Sending POST request to {}", url);
            
                let link = ctx.link().clone();
                spawn_local(async move {
                    let payload = serde_json::json!({
                        "id": id,
                        "password": password
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
                            log::error!("Network error: {:?}", err);
                            link.send_message(Msg::ReceiveResponse(Err(format!("Network error: {:?}", err))));
                            return;
                        }
                    };
            
                    if response.ok() {
                        let json = match wasm_bindgen_futures::JsFuture::from(response.json().unwrap()).await {
                            Ok(data) => data,
                            Err(err) => {
                                log::error!("Failed to parse JSON: {:?}", err);
                                link.send_message(Msg::ReceiveResponse(Err(format!("JSON parse error: {:?}", err))));
                                return;
                            }
                        };
            
                        let token = json.as_string().unwrap_or("No token".to_string());
                        log::info!("Login succeeded. Token: {}", token);
                        link.send_message(Msg::ReceiveResponse(Ok(token)));
            
                        // Redirect to Home
                        let navigator = link.navigator().unwrap();
                        navigator.push(&AppRoute::Home);
                    } else {
                        let error_message = format!("Status: {}, Message: {}", response.status(), response.status_text());
                        log::error!("Login failed: {}", error_message);
                        link.send_message(Msg::ReceiveResponse(Err(error_message)));
                    }
                });
            
                false
            }
            
            
            
            Msg::ReceiveResponse(result) => {
                match result {
                    Ok(..) => {
                        self.show_modal = false; 
                    }
                    Err(..) => {
                        self.show_modal = true;    
                    }
                }
                true
            }
            Msg::CloseModal => {
                self.show_modal = false; // Close the modal
                true
            }
            
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="page-background">
                <div class="logo-container">
                    <Link<AppRoute> to={AppRoute::Home}>
                        <img src={logo_src()} alt="Logo" class="logo" />
                    </Link<AppRoute>>
                    <div class="logo-divider"></div> // Divider line
                </div>
                
                <div class="center-container">
                    <div class="floating-window">                        
                        <form onsubmit={ctx.link().callback(|e: SubmitEvent| {
                            e.prevent_default();
                            Msg::Submit})}>
                            <label for="id" class="form-label">{"ID"}</label>
                            <input
                                type="text"
                                id="id"
                                name="id"
                                value={self.id.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| Msg::UpdateId(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                class="form-input"/>

                            <label for="password" class="form-label">{"Password"}</label>
                            <input
                                type="password"
                                id="password"
                                name="password"
                                value={self.password.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| Msg::UpdatePassword(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                class="form-input"/>  

                            <button type="submit" class="built-in-button"> {"LOGIN"} </button>

                            <div style="text-align: center;">
                                <Link<AppRoute> to={AppRoute::Register} classes="built-in-link">
                                    { "Sign Up" }
                                </Link<AppRoute>>
                            </div>
                        </form>
                    </div>
                    // Add a modal to show an error message when login fails
                    if self.show_modal {
                        <div class="modal">
                            <p>{ "Invalid ID or password" }</p>
                            <button onclick={ctx.link().callback(|_| Msg::CloseModal)} class="floating-window-button"> { "Close" } </button>
                        </div>
                    }
                </div>
            </div>
        }
    }
}

use yew::prelude::*;
use yew_router::prelude::*;
use yew::platform::spawn_local;
use serde_json::Value;
use gloo_net::http::Request;
use crate::app::AppRoute;
use crate::config::*;

pub struct Register {
    id: String,
    password: String,
    message: String,
    show_modal: bool,
}

pub enum Msg {
    UpdateId(String),
    UpdatePassword(String),
    Submit,
    ReceiveResponse(Result<String, String>),
    CloseModal,
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Register {
            id: String::new(),
            password: String::new(),
            message: String::new(),
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
                let _url = format!("{}/user/register", api_base());

                log::info!("Sending POST request to {}", _url);
                
                let link = ctx.link().clone();
                spawn_local(async move {
                    let payload = serde_json::json!({
                        "id": id,
                        "password": password
                    });
                    let request_builder = Request::post(&_url)
                        .header("Content-Type", "application/json")
                        .body(payload.to_string());
                    
                    match request_builder {
                        Ok(request) => {
                            let response = request.send().await;
                
                            match response {
                                Ok(resp) if resp.status() == 200 => {
                                    let json: Value = resp.json().await.unwrap_or(Value::Null);
                                    let token = json["Token"].as_str().unwrap_or("No token").to_string();
                
                                    log::info!("Sign up succeeded. Token: {}", token);
                                    link.send_message(Msg::ReceiveResponse(Ok(token)));

                                    // Redirect to login page
                                    let navigator = link.navigator().unwrap(); // Get the navigator instance
                                    navigator.push(&AppRoute::Login);
                                }
                                Ok(resp) => {
                                    let status = resp.status();
                                    let error_body = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                                    log::error!("Sign up failed with status {}: {}", status, error_body);
                                    link.send_message(Msg::ReceiveResponse(Err(format!(
                                        "Error: {}, Details: {}",
                                        status, error_body
                                    ))));
                                }
                                Err(err) => {
                                    log::error!("Network error: {}", err);
                                    link.send_message(Msg::ReceiveResponse(Err(err.to_string())));
                                }
                            }
                        }
                        Err(err) => {
                            log::error!("Request build error: {}", err);
                            link.send_message(Msg::ReceiveResponse(Err(err.to_string())));
                        }
                    }
                });
                  

                false
            }
            Msg::ReceiveResponse(result) => {
                match result {
                    Ok(..) => {
                        self.show_modal = false; 
                    }
                    Err(msg) => {
                        // Extract the JSON part from the error message
                        if let Some(start) = msg.find('{') {
                            let json_part = &msg[start..]; // Slice starting from the first `{`
                            match serde_json::from_str::<Value>(json_part) {
                                Ok(parsed) => {
                                    log::info!("Parsed JSON: {:?}", parsed);

                                    // Extract the `error` field
                                    if let Some(error_message) = parsed.get("error").and_then(|v| v.as_str()) {
                                        self.message = error_message.to_string();
                                    } else {
                                        log::error!("`error` field missing or not a string.");
                                        self.message = "An unexpected error occurred.".to_string();
                                    }
                                }
                                Err(err) => {
                                    log::error!("Failed to parse JSON: {}", err);
                                    self.message = "An unexpected error occurred.".to_string();
                                }
                            }
                        } else {
                            log::error!("No JSON found in the message: {}", msg);
                            self.message = "An unexpected error occurred.".to_string();
                        }
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
                            
                            <div class="floating-window-title">
                                { "Sign Up" }
                            </div>

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

                            <button type="submit" class="built-in-button"> {"Register"} </button>
                        </form>
                    </div>
                    if self.show_modal {
                        <div class="modal">
                            <p>{ &self.message }</p>
                            <button onclick={ctx.link().callback(|_| Msg::CloseModal)} class="floating-window-button"> { "Close" } </button>
                        </div>
                    }
                </div>
            </div>
        }
    }
}

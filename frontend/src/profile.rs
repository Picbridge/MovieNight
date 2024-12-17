use yew::prelude::*;
//use yew::platform::spawn_local;
//use gloo_net::http::Request;
//use serde_json::Value;
use yew_router::prelude::*;
//use crate::app::APIBASE;
use crate::app::AppRoute;
use crate::auth_context::SharedAuthContext;
//use crate::components::sidebar::Sidebar;

pub struct Profile {
    auth_context: SharedAuthContext,
    _context_handle: ContextHandle<SharedAuthContext>,
}

pub enum Msg {
    ContextChanged(SharedAuthContext),
}

impl Component for Profile {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (auth_context, context_handle) = ctx
            .link()
            .context::<SharedAuthContext>(ctx.link().callback(Msg::ContextChanged))
            .expect("No AuthContext provided");

        let profile = Profile {
            auth_context,
            _context_handle: context_handle,
        };

        profile
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            
            Msg::ContextChanged(new_context) => {
                self.auth_context = new_context;
                true 
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="page-background">
                // Logo Section
                <div class="logo-container">
                    <Link<AppRoute> to={AppRoute::Home}>
                        <img src="logo.png" alt="Logo" class="logo" />
                    </Link<AppRoute>>
                    <div class="logo-divider"></div> // Divider line
                </div>
                <h1>{ "Profile" }</h1>

                // Sidebar Component
                //<Sidebar />

                // Main Content Section
                <div class="main-content">
                    <h1>{ "Profile" }</h1>
                    <p>{ "A button that allows sidebar to show up" }</p>
                    <p style="color: red;">{ "when pressed: Gets unfolded" }</p>
                    <p style="color: green;">{ "when pressed again: Gets folded" }</p>
                </div>
            </div>
        }
    }
}
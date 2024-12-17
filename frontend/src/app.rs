use yew::prelude::*;
use yew_router::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::auth_context::{AuthContext, SharedAuthContext};
use crate::home::Home;
use crate::login::Login;
use crate::signup::Register;
use crate::profile::Profile;
use crate::recommendation::Recommendation;

//pub const APIBASE: &str = "http://localhost:5173/api";//"https://movienight-backend.purplebay-46d91d82.westus2.azurecontainerapps.io/api";//

pub struct App {
    auth_context: SharedAuthContext,
}

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/recommendation")]
    Recommendation,
    #[at("/profile")]
    Profile,
    #[at("/signup")]
    Register,
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let auth_context = Rc::new(RefCell::new(AuthContext::new()));

        Self { auth_context }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log::info!("Rendering App component");
        html! {
            <ContextProvider<SharedAuthContext> context={self.auth_context.clone()}>
                <BrowserRouter>
                    <Switch<AppRoute> render={switch} />
                </BrowserRouter>
            </ContextProvider<SharedAuthContext>>
        }
    }
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Register => html! { <Register /> },
        AppRoute::Login => html! { <Login /> },
        AppRoute::Home => html! { <Home /> },
        AppRoute::Profile => html! { <Profile /> },
        AppRoute::Recommendation => html! { <Recommendation /> },
    }
}

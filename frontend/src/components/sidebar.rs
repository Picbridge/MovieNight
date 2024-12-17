use yew::prelude::*;
//use yew_router::prelude::*;
//use crate::app::AppRoute;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub children: Children, // Allow passing child HTML content
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let is_open = use_state(|| false);

    let toggle_sidebar = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class="sidebar-container">
            // Toggle Button
            <button onclick={toggle_sidebar} class={classes!("toggle-btn", if *is_open { "inside" } else { "outside" })}>
                { if *is_open { ">" } else { "<" } }
            </button>

            // Sidebar
            <div class={classes!("sidebar", if *is_open { "open" } else { "closed" })}>
                <div class="sidebar-content">
                    <div style="text-align: center;">
                        { for props.children.iter() } 
                    </div>
                </div>
            </div>
        </div>
    }
}

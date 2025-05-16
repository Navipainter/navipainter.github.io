use yew::prelude::*;
use yew_router::prelude::*;
use serde::Deserialize;

pub mod routes;
use routes::{switch, Route};

const GALLERIES_JSON: &str = include_str!("../../static/galleries.json");

#[derive(Deserialize, Clone, PartialEq)]
pub struct RawGallery {
    pub name: String,
    pub slug: String,
}

#[derive(Properties, PartialEq)]
struct SidebarProps {
    pub galleries: Vec<RawGallery>,
    pub mobile_open: bool,
    pub toggle_mobile: Callback<MouseEvent>,
}

#[function_component(Sidebar)]
fn sidebar(props: &SidebarProps) -> Html {
    let location = use_location().unwrap();
    let current_path = location.path();

    html! {
        <nav class={classes!("my-sidebar", if props.mobile_open { "open" } else { "" })}>
            // Home
            <div
                class={classes!("my-nav-item", if current_path == "/" { "active" } else { "" })}
                onclick={props.toggle_mobile.clone()}
            >
                <Link<Route> to={Route::Home}>{ "Inicio" }</Link<Route>>
            </div>
            // Galleries
            {
                for props.galleries.iter().map(|g| {
                    let slug = g.slug.clone();
                    let path = format!("/gallery/{slug}", slug = slug);
                    html! {
                        <div
                            class={classes!(
                                "my-nav-item",
                                if current_path == path { "active" } else { "" }
                            )}
                            onclick={props.toggle_mobile.clone()}
                        >
                            <Link<Route> to={Route::Gallery { name: slug.clone() }}>
                                { &g.name }
                            </Link<Route>>
                        </div>
                    }
                })
            }
            // About
            <div
                class={classes!("my-nav-item", if current_path == "/about" { "active" } else { "" })}
                onclick={props.toggle_mobile.clone()}
            >
                <Link<Route> to={Route::About}>{ "Sobre mí" }</Link<Route>>
            </div>
            // Proyectos
            <div
                class={classes!("my-nav-item", if current_path == "/proyectos" { "active" } else { "" })}
                onclick={props.toggle_mobile.clone()}
            >
                <Link<Route> to={Route::Proyectos}>{ "Proyectos" }</Link<Route>>
            </div>
        </nav>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let galleries: Vec<RawGallery> = serde_json::from_str(GALLERIES_JSON)
        .expect("Invalid galleries.json");

    let mobile_open = use_state(|| false);
    let toggle_mobile = {
        let mo = mobile_open.clone();
        Callback::from(move |_| mo.set(!*mo))
    };
    let close_mobile = {
        let mo = mobile_open.clone();
        Callback::from(move |_| if *mo { mo.set(false) })
    };

    html! {
        <BrowserRouter>
            <div class="my-app-layout">
                <Sidebar
                    galleries={galleries.clone()}
                    mobile_open={*mobile_open}
                    toggle_mobile={toggle_mobile.clone()}
                />
                <button class="hamburger" onclick={toggle_mobile}>
                    { if *mobile_open { "✕" } else { "☰" } }
                </button>
                <main class={classes!("my-content", if *mobile_open { "open" } else { "" })}
                      onclick={close_mobile}>
                    <Switch<Route> render={|route| switch(route)} />
                </main>
            </div>
        </BrowserRouter>
    }
}

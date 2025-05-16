// src/app/routes/mod.rs
use yew::prelude::*;         // brings in html! macro
use yew_router::prelude::*;

mod about;
mod home;
mod proyectos;
mod gallery;

pub use about::About;
pub use home::Home;
pub use proyectos::Proyectos;
pub use gallery::GalleryPage;

/// Application routes
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/gallery/:name")]
    Gallery { name: String },

    #[at("/about")]
    About,

    #[at("/proyectos")]
    Proyectos,

    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Routing switch: consumes Route and returns corresponding Html
pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Gallery { name } => html! { <GalleryPage gallery_name={name} /> },
        Route::About => html! { <About /> },
        Route::Proyectos => html! { <Proyectos /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}

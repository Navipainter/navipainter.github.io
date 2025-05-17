use yew::prelude::*;
use yew_router::prelude::*;

mod about;
mod home;
mod gallery;

pub use about::About;
pub use home::Home;
pub use gallery::GalleryPage;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/gallery/:name")]
    Gallery { name: String },

    #[at("/about")]
    About,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Gallery { name } => html! { <GalleryPage gallery_name={name} /> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}

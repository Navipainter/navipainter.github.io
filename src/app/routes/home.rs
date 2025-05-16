use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
          <h1>{ "Bienvenido a mi portfolio" }</h1>
          <p>{ "Selecciona una galería desde el menú superior." }</p>
        </>
    }
}

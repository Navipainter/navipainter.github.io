use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section class="my-section">
            <div class="my-wrapper">
                <div class="my-content">
                    <h1>{"Sobre mí"}</h1>
                    <p>{"Soy Navi Painter."}</p>
                </div>
            </div>
        </section>
    }
}

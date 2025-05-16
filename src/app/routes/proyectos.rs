use yew::prelude::*;

#[function_component(Proyectos)]
pub fn proyectos() -> Html {
    html! {
        <section class="my-section">
            <div class="my-wrapper">
                <div class="my-content">
                    <h1>{"Mis proyectos"}</h1>
                    <ul class="my-list">
                        <li class="my-list-item">{"Proyecto A"}</li>
                        <li class="my-list-item">{"Proyecto B"}</li>
                    </ul>
                </div>
            </div>
        </section>
    }
}


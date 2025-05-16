mod app;

use app::App;
use yew::Renderer;

fn main() {
    // Inicia la app registrando el componente raiz
    Renderer::<App>::new().render();
}

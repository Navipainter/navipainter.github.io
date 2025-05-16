use yew::prelude::*;
use serde::Deserialize;

const GALLERIES_JSON: &str = include_str!("../../../static/galleries.json");

#[derive(Deserialize, Clone)]
pub struct ImageData {
    pub url: String,
    pub alt: String,
}

#[derive(Deserialize, Clone)]
pub struct RawGallery {
    #[allow(dead_code)]
    pub name: String,
    pub slug: String,
    pub images: Vec<ImageData>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub gallery_name: String,
}

#[function_component(GalleryPage)]
pub fn gallery_page(props: &Props) -> Html {
    let Props { gallery_name } = props;

    let all_galleries: Vec<RawGallery> =
        serde_json::from_str(GALLERIES_JSON).expect("Invalid galleries.json");

    let gallery = all_galleries.into_iter()
        .find(|g| g.slug == *gallery_name)
        .unwrap_or(RawGallery {
            name: gallery_name.clone(),
            slug: gallery_name.clone(),
            images: Vec::new(),
        });

    let modal_open = use_state(|| false);
    let current_index = use_state(|| 0_usize);

    let open_modal = {
        let modal_open = modal_open.clone();
        let current_index = current_index.clone();
        Callback::from(move |idx: usize| {
            current_index.set(idx);
            modal_open.set(true);
        })
    };
    let close_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| modal_open.set(false))
    };
    let next_image = {
        let current_index = current_index.clone();
        let total = gallery.images.len();
        Callback::from(move |_| {
            current_index.set((*current_index + 1) % total);
        })
    };
    let prev_image = {
        let current_index = current_index.clone();
        let total = gallery.images.len();
        Callback::from(move |_| {
            current_index.set((*current_index + total - 1) % total);
        })
    };

    html! {
        <>
            <section class="my-gallery">
                {
                    if gallery.images.is_empty() {
                        html! { <p>{ "No images in this gallery" }</p> }
                    } else {
                        gallery.images.iter().enumerate().map(|(i, img)| {
                            let open = open_modal.clone();
                            html! {
                                <div class="my-gallery-item"
                                     onclick={Callback::from(move |_| open.emit(i))}>
                                    <img src={img.url.clone()} alt={img.alt.clone()} />
                                </div>
                            }
                        }).collect::<Html>()
                    }
                }
            </section>

            { if *modal_open {
                html! {
                    <div id="my-modal"
                         class="my-modal show"
                         onclick={close_modal.clone()}>
                      <div class="my-modal-content"
                           onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <div class="modal-body">
                          <button class="my-modal-prev"
                                  onclick={prev_image.clone()}>{ "❮" }</button>

                          <img
                            src={gallery.images[*current_index].url.clone()}
                            alt={gallery.images[*current_index].alt.clone()} />

                          <button class="my-modal-close"
                                  onclick={close_modal.clone()}>{ "✖" }</button>

                          <button class="my-modal-next"
                                  onclick={next_image.clone()}>{ "❯" }</button>
                        </div>
                      </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </>
    }
}

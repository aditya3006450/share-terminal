use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct ArtImage {
    src: String,
    color: String,
    title: String,
}

#[function_component(PixelArt)]
pub fn pixelart() -> Html {
    let images = Rc::new(vec![
        ArtImage {
            src: "/public/RiverBy.gif".to_string(),
            color: "#3a86ff".to_string(),
            title: "".to_string(),
        },
        ArtImage {
            src: "/public/WaterTow.jpg".to_string(),
            color: "#ff006e".to_string(),
            title: "".to_string(),
        },
        ArtImage {
            src: "/public/Casel.gif".to_string(),
            color: "#8338ec".to_string(),
            title: "".to_string(),
        },
        ArtImage {
            src: "/public/Tree.jpg".to_string(),
            color: "#fb5607".to_string(),
            title: "".to_string(),
        },
    ]);

    let current_index = use_state(|| 0);

    let on_next = {
        let current_index = current_index.clone();
        let images_len = images.len();
        Callback::from(move |_| {
            let next = (*current_index + 1) % images_len;
            current_index.set(next);
        })
    };

    let on_prev = {
        let current_index = current_index.clone();
        let images_len = images.len();
        Callback::from(move |_| {
            let prev = if *current_index == 0 {
                images_len - 1
            } else {
                *current_index - 1
            };
            current_index.set(prev);
        })
    };

    let current_image = if let Some(img) = images.get(*current_index) {
        img.clone()
    } else {
        images
            .first()
            .unwrap_or(&ArtImage {
                src: "".to_string(),
                color: "#4361ee".to_string(),
                title: "Default".to_string(),
            })
            .clone()
    };

    html! {
        <div class="pixel_art_container">
            <div class="pixel_art_frame" style={format!("--accent-color: {}", current_image.color)}>

                <div class="pixel_art_display">
                    if current_image.src.is_empty() {
                        <div class="pixel_art_placeholder" style={format!("background-color: {}", current_image.color)}>
                            <span>{"Pixel Art"}</span>
                        </div>
                    } else {
                        <img src={current_image.src} alt={current_image.title.clone()} class="pixel_art_image" />
                    }
                </div>

                <div class="pixel_art_navigation">
                    <button class="nav_button prev_button" onclick={on_prev}>
                        <span class="arrow_icon">{"←"}</span>
                    </button>
                    <button class="nav_button next_button" onclick={on_next}>
                        <span class="arrow_icon">{"→"}</span>
                    </button>
                </div>
            </div>
        </div>
    }
}


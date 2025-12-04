use crate::components::canvas_topbar::CanvasTopBar;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CanvasPageProps {
    pub id: String,
    pub dark_mode: bool,
    pub toggle_theme: Callback<()>,
}

#[function_component(CanvasPage)]
pub fn canvas_page(props: &CanvasPageProps) -> Html {
    let theme_class = if props.dark_mode { "dark-theme" } else { "" };

    html! {
        <div class={theme_class}>
            <CanvasTopBar dark_mode={props.dark_mode} toggle_theme={props.toggle_theme.clone()} />
            <div class="canvas-container">
                <canvas id="canvas"></canvas>
            </div>
        </div>
    }
}

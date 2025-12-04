use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CanvasTopBarProps {
    pub dark_mode: bool,
    pub toggle_theme: Callback<()>,
}

#[function_component(CanvasTopBar)]
pub fn canvas_top_bar(props: &CanvasTopBarProps) -> Html {
    let navigator = use_navigator().unwrap();

    let on_disconnect_click = Callback::from(move |_| {
        navigator.back();
    });

    html! {
        <div class="canvas-topbar">
            <div class="canvas-topbar-content">
                <div class="canvas-topbar-actions">
                    <button class="theme-toggle" onclick={props.toggle_theme.reform(|_: MouseEvent| ())} title="Toggle theme">
                        {
                            if props.dark_mode {
                                html! { <Icon icon_id={IconId::BootstrapSunFill} /> }
                            } else {
                                html! { <Icon icon_id={IconId::BootstrapMoonFill} /> }
                            }
                        }
                    </button>
                    <button onclick={on_disconnect_click} class="btn-disconnect">
                        { "Disconnect" }
                    </button>
                </div>
            </div>
        </div>
    }
}

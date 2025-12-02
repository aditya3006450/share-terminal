use yew::prelude::*;

#[function_component(RequestAccess)]
pub fn request_access() -> Html {
    html! {
        <div class="main-content">
            <div class="card">
                <h1>{"Request Access"}</h1>
                <p>{"This is the Request Access page. Here you can request access to other terminals."}</p>
                // Add your request access form or content here
                <form>
                    <input type="text" class="input-field" placeholder="Enter user ID or terminal name" />
                    <button type="submit" class="submit-btn">{"Send Request"}</button>
                </form>
            </div>
        </div>
    }
}
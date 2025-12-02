use yew::prelude::*;

#[function_component(OutgoingRequests)]
pub fn outgoing_requests() -> Html {
    html! {
        <div class="main-content">
            <div class="card">
                <h1>{"Outgoing Requests"}</h1>
                <p>{"This page shows the status of access requests you have sent to other users."}</p>
                // Example list of outgoing requests
                <div class="request-item">
                    <span>{"Request to User Bob: Pending"}</span>
                </div>
                <div class="request-item">
                    <span>{"Request to User Alice: Accepted"}</span>
                </div>
            </div>
        </div>
    }
}
use yew_router::prelude::*;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/access")]
    Access,
    #[at("/access/connected-users")]
    ConnectedUsers,
    #[at("/access/viewers")]
    Viewers,
    #[at("/access/incoming-requests")]
    IncomingRequests,
    #[at("/access/outgoing-requests")]
    OutgoingRequests,
    #[at("/access/request-access")]
    RequestAccess,
}

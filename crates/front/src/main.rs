use yew_router::prelude::*;
use yew::prelude::*;

use reqwasm::http::Request;
use serde::Deserialize;


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[at("/secure")]
    Secure,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}

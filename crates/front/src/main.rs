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
#[function_component(Home)]
fn home() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Secure));
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "Go Secure" }</button>
        </div>
    }
}
#[function_component(NotFound)]
fn not_found() -> Html {
    let history = use_history().unwrap();
    let go_home_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(Route::Home));
        html! {
            <button {onclick}>{"Home"}</button>
        }
    };

    let go_secure_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(Route::Secure));
        html! {
            <button {onclick}>{"Secure"}</button>
        }
    };
    html! {
        <>
            {go_home_button}
            {go_secure_button}
        </>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
             <Home />
         },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! {
             <NotFound />
        },
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

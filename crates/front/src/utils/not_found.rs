use yew::{
    function_component,
    use_effect_with_deps,
    html,
    Component,
    Context,
    Properties,
    use_state,
};
use serde::Deserialize;
use reqwasm::http::Request;

#[derive(Clone, PartialEq, Deserialize, Debug)]
struct UserDetail {
    id: u64,
    first_name: String,
    last_name: String,
    types: i16,
    gender: String,
    device: String,
    language: String,
    perm: i16,
    link: String,
    city: Option<String>,
    status: Option<String>,
    image: Option<String>,
    birthday: String,
    last_activity: String,
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let test = use_state(|| UserDetail {
        id: 0,
        first_name: "".to_string(),
        last_name: "".to_string(),
        types: 0,
        gender: "".to_string(),
        device: "".to_string(),
        language: "".to_string(),
        perm: 0,
        link: "".to_string(),
        city: None,
        status: None,
        image: None,
        birthday: "".to_string(),
        last_activity: "".to_string(),
    });
    {
        let test = test.clone();
        use_effect_with_deps(move |_| {
            let test = test.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_test: UserDetail = Request::get("/api_users/v1/users/1")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                test.set(fetched_test);
            });
            || ()
        }, ());
    }

    html!{
        <section  class="section">
            <h2 class="title">{"404 Page not found"}</h2>
            { test.id }
            <a href="/" class="button is-primary">{"back"}</a>
        </section>
    }

}

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
use crate::utils::requests::request_get;

#[derive(Clone, PartialEq, Deserialize)]
struct Test {
    text: String,
}
#[function_component(NotFound)]
pub fn not_found() -> Html {
    let test = use_state(|| Test {text: "".to_string()});
    {
        let test = test.clone();
        use_effect_with_deps(move |_| {
            let test = test.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_test: Test = request_get::<Test>("/api/users/user/1".to_string()).await;
                test.set(fetched_test);
            });
            || ()
        }, ());
    }

    html!{
        <section  class="section">
            <h2 class="title">{"404 Page not found"}</h2>
            { test }
            <a href="/" class="button is-primary">{"back"}</a>
        </section>
    }

}

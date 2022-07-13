use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;


#[derive(Clone, PartialEq, Deserialize)]
struct TestData {
    name:        String,
    description: String,
}

#[function_component(App)]
fn app() -> Html {
    let test = use_state(|| vec![]);
    {
        let test = test.clone();
        use_effect_with_deps(move |_| {
            let test = test.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_test: Vec<TestData> = Request::get("/api/v1/test/")
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

    html! {
        <>
        <h1>{ "Первый тест" }</h1>
        <div>
          <h3>{format!("{}", test.name)}</h3>
          <p>{format!("{}", test.description)}</p>
        </div>
    </>
    }
}

fn main() {
    yew::start_app::<App>();
}

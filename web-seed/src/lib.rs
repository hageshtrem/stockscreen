use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

const API_URL: &str = env!("API_URL"); // "http://localhost/v1"

#[derive(Serialize)]
struct Model {
    name: String,
    description: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Reply {
    message: String,
}

#[derive(Clone)]
enum Msg {
    NameChanged(String),
    DescriptionChanged(String),
    Submit,
    Submitted(fetch::ResponseDataResult<Reply>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::NameChanged(name) => model.name = name,
        Msg::DescriptionChanged(description) => model.description = description,
        Msg::Submit => {
            log!("{}", API_URL);
            let request = Request::new(format!("{}/emitent", API_URL))
                .method(Method::Post)
                .send_json(model);
            orders.perform_cmd(async { request.fetch_json_data(Msg::Submitted).await });
        }
        Msg::Submitted(Ok(response)) => log!(response),
        Msg::Submitted(Err(error)) => {
            log!("--------ERROR occured!-------");
            log!(error);
        }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    form![
        style! {
            St::Display => "flex",
            St::FlexDirection => "column",
        },
        label!["Name"],
        input![
            attrs! {
                At::Value => model.name
            },
            input_ev(Ev::Input, Msg::NameChanged)
        ],
        label!["Description"],
        input![
            attrs! {
                At::Value => model.description,
            },
            input_ev(Ev::Input, Msg::DescriptionChanged)
        ],
        button![
            "Submit",
            ev(Ev::Click, |event| {
                event.prevent_default();
                Msg::Submit
            })
        ],
        "Note: Errors are logged into the console log.",
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}

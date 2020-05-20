use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

const API_URL: &str = env!("API_URL"); // "http://localhost/v1"

#[derive(Serialize)]
struct Model {
    name: String,
    description: String,
    emitents: Vec<Emitent>,
    error: Option<String>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            emitents: vec![],
            error: None,
        }
    }
}

#[derive(Serialize, Clone)]
struct NewEmitentRequest {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize, Clone)]
struct NewEmitentReply {
    message: String,
}

#[derive(Debug, Deserialize, Clone)]
struct GetEmitentsReply {
    results: Vec<Emitent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Emitent {
    id: String, // https://developers.google.com/protocol-buffers/docs/proto3#json
    name: String,
    description: String,
}

enum Msg {
    NameChanged(String),
    DescriptionChanged(String),
    FetchData,
    DataFetched(fetch::Result<GetEmitentsReply>),
    Submit,
    Submitted(fetch::Result<NewEmitentReply>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchData => {
            async fn do_request() -> fetch::Result<GetEmitentsReply> {
                fetch(format!("{}/emitent", API_URL))
                    .await?
                    .check_status()?
                    .json()
                    .await
            }
            orders.perform_cmd(async { Msg::DataFetched(do_request().await) });
        }
        Msg::DataFetched(Ok(ger)) => model.emitents = ger.results,
        Msg::DataFetched(Err(error)) => log!(error),
        Msg::NameChanged(name) => model.name = name,
        Msg::DescriptionChanged(description) => model.description = description,
        Msg::Submit => {
            let newemitent = NewEmitentRequest {
                name: model.name.clone(),
                description: model.description.clone(),
            };
            async fn send_message(req: NewEmitentRequest) -> fetch::Result<NewEmitentReply> {
                Request::new(format!("{}/emitent", API_URL))
                    .method(Method::Post)
                    .json(&req)?
                    .fetch()
                    .await?
                    .check_status()?
                    .json()
                    .await
            }
            orders.perform_cmd({ async { Msg::Submitted(send_message(newemitent).await) } });
        }
        Msg::Submitted(Ok(response)) => {
            log!(response);
            orders.send_msg(Msg::FetchData);
        }
        Msg::Submitted(Err(error)) => {
            log!(error);
            model.error = Some(format!("{:?}", error));
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    let table_body = model
        .emitents
        .iter()
        .map(|elem| {
            tr![
                td![elem.id.clone()],
                td![elem.name.clone()],
                td![elem.description.clone()],
            ]
        })
        .collect::<Vec<Node<Msg>>>();
    div![
        style! {
            St::Display => "grid"
            St::GridTemplateColumns => "50% 50%"
        },
        div![table![
            tr![th!["#"], th!["Name"], th!["Description"],],
            table_body,
        ]],
        div![
            form![
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
            ],
            if let Some(err_msg) = &model.error {
                div![err_msg]
            } else {
                empty![]
            },
        ],
    ]
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.send_msg(Msg::FetchData);
    AfterMount::default()
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}

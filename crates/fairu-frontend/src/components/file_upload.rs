use fairu_entity::fairu::Model;
use gloo_net::http::Request;
use indexmap::indexmap;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{window, File, FormData, HtmlInputElement};
use yew::prelude::*;
use yew::{function_component, html, use_reducer, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[derive(Deserialize, Serialize)]
pub struct FileUploadResponse {
    pub uuid: String,
}

// Reducer's action
enum FileAction {
    SetFile(Option<File>),
    SetExpireAfter(String),
    Submit,
}

// Reducer's state
#[derive(Clone, PartialEq)]
struct FileState {
    file: Option<File>,
    expire_after: String,
}

impl Default for FileState {
    fn default() -> Self {
        Self {
            file: None,
            expire_after: "86400".to_string(),
        }
    }
}

impl Reducible for FileState {
    type Action = FileAction;

    fn reduce(self: Rc<FileState>, action: Self::Action) -> Rc<FileState> {
        match action {
            FileAction::SetFile(file) => Rc::new(Self {
                file,
                ..self.as_ref().clone()
            }),
            FileAction::SetExpireAfter(expire_after) => Rc::new(Self {
                expire_after,
                ..self.as_ref().clone()
            }),
            FileAction::Submit => self,
        }
    }
}

#[function_component(FileUpload)]
pub fn file_upload() -> Html {
    let file_data = use_reducer(|| FileState::default());

    // Expire_after key/value hashmap
    let expire_after_values = indexmap! {
        "1800" => "30 minutes",
        "21600" => "6 hours",
        "86400" => "1 day",
        "604800" => "1 week",
        "2592000" => "1 month",
        "0" => "forever",
    };

    let update = {
        let file_data = file_data.clone();

        Callback::from(move |action: FileAction| match action {
            FileAction::SetFile(file) => {
                file_data.dispatch(FileAction::SetFile(file.clone()));
            }
            FileAction::SetExpireAfter(expire_after) => {
                file_data.dispatch(FileAction::SetExpireAfter(expire_after.clone()));
            }
            FileAction::Submit => {
                let file = file_data.file.clone();
                let expire_after = file_data.expire_after.clone();

                // If file is not set, do nothing
                if file.is_none() {
                    return;
                }

                // Send file to backend using gloo_net and multipart async
                wasm_bindgen_futures::spawn_local(async move {
                    let file = file.unwrap();

                    let form_data = FormData::new().unwrap();
                    form_data
                        .append_with_blob_and_filename("file", &file, &file.name())
                        .unwrap();
                    form_data
                        .append_with_str("expire_after", &expire_after.to_string())
                        .unwrap();

                    let response = Request::post("http://localhost:8080/api/files")
                        .header("Content-Type", "multipart/form-data")
                        .body(form_data)
                        .unwrap()
                        .send()
                        .await
                        .unwrap();

                    let response: FileUploadResponse = response.json().await.unwrap();

                    // Redirect to file page
                    window()
                        .unwrap()
                        .location()
                        .set_href(&format!("/files/{}", response.uuid))
                        .unwrap();
                });

                // Reset state
                file_data.dispatch(FileAction::SetFile(None));
                file_data.dispatch(FileAction::SetExpireAfter("86400".to_string()));
            }
        })
    };

    html! {
        <div>
            <form onsubmit={
                let update = update.clone();
                Callback::from(move |e: SubmitEvent| {
                    e.prevent_default();
                    update.emit(FileAction::Submit);
                })
            } enctype="multipart/form-data">
                <div class="mb-3">
                    <label class="form-label" for="content">{ "File" }</label>
                    <input class="form-control" id="content" name="content" type="file" onchange={
                        let update = update.clone();
                        Callback::from(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let file = File::from(input.files().unwrap().get(0).unwrap());
                            update.emit(FileAction::SetFile(Some(file)));
                        })
                    } />
                </div>

                <div class="mb-3">
                    <label for="expire_after" class="form-label">{"Expire"}</label>
                    <select
                        id="expire_after"
                        name="expire_after"
                        class="form-control"
                        onchange={
                            let update = update.clone();
                            Callback::from(move |e: Event| {
                                let value = e.target_unchecked_into::<HtmlInputElement>().value();
                                update.emit(FileAction::SetExpireAfter(value));
                            })
                        }>
                        { for expire_after_values.iter().map(|(key, value)| {
                            let selected = *key == "86400";
                            html! {
                                <option value={key.to_string()} selected={selected}>{value}</option>
                            }
                        }) }
                    </select>
                </div>

                <div class="mb-3">
                    <button class="btn btn-primary" type="submit">{ "Submit" }</button>
                </div>
            </form>
        </div>
    }
}

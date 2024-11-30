use yew::prelude::*;
use crate::components::file_upload::FileUpload;

pub struct NewFile;

impl Component for NewFile {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="mx-auto max-w-3xl px-2 sm:px-6 lg:px-8">
                <h1 class="title">
                    { "New file" }
                </h1>

                <FileUpload />
            </section>
        }
    }
}

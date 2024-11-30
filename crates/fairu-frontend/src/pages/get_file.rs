use crate::app::Route;
use yew::prelude::*;
use yew_router::components::Link;

pub struct GetFile;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub uuid: String,
}

impl Component for GetFile {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let _uuid = ctx.props().uuid.clone();

        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="mx-auto max-w-3xl px-2 sm:px-6 lg:px-8">
                <h1 class="title">{ "Fairu is a file upload service written in Rust." }</h1>
                <p class="mb-6">{ "It's open source and available on GitHub." }</p>
                <div class="mb-6">
                    <Link<Route> classes="btn btn-primary" to={Route::NewFile}>
                        { "Upload file" }
                    </Link<Route>>
                </div>
            </section>
        }
    }
}

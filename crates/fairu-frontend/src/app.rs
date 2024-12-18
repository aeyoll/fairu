use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::route_link::RouteLink;
use crate::pages::about::About;
use crate::pages::get_file::GetFile;
use crate::pages::new_file::NewFile;
use crate::pages::page_not_found::PageNotFound;
use yew::html::Scope;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/file/:uuid")]
    GetFile { uuid: String },
    #[at("/about")]
    About,
    #[at("/")]
    NewFile,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {}

pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <div>
                    { self.view_nav(ctx.link()) }

                    <main>
                        <Switch<Route> render={switch} />
                    </main>
                </div>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            <nav class="bg-green-800 mb-3">
                <div class="mx-auto max-w-3xl px-2 sm:px-6 lg:px-8">
                    <div class="relative flex h-16 items-center justify-between">
                        <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                            <div class="flex flex-shrink-0 items-center space-x-4">
                                <img src="/static/logo.svg" alt="Fairu logo" width="24" height="24" />
                                <RouteLink to={Route::NewFile}>
                                    <div class="flex items-center text-white gap-2">
                                        <img src="/static/plus.svg" alt="New file" width="18" height="18" />
                                        { "New file" }
                                    </div>
                                </RouteLink>
                            </div>
                            <div class="hidden  sm:block sm:ml-auto">
                                <div class="flex space-x-4">
                                    <RouteLink to={Route::About}>
                                        <div class="flex items-center text-white gap-2">
                                            <img src="/static/info.svg" alt="About" width="18" height="18" />
                                            { "About" }
                                        </div>
                                    </RouteLink>

                                    <a href="https://github.com/aeyoll/fairu" target="_blank" rel="noopener" class="rounded-md px-3 py-2 text-sm font-medium text-white">
                                        <div class="flex items-center text-white gap-2">
                                            <img src="/static/github.svg" alt="Github" width="18" height="18" />
                                            { "Github" }
                                        </div>
                                    </a>

                                    <span class="rounded-md px-3 py-2 text-sm font-medium text-gray-300">
                                        { format!("v{}", env!("CARGO_PKG_VERSION")) }
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::About => {
            html! { <About /> }
        }
        Route::GetFile { uuid } => {
            html! { <GetFile uuid={uuid} /> }
        }
        Route::NewFile => {
            html! { <NewFile /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

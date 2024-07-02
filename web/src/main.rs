use leptos::*;
use leptos_router::*;
use stylance::import_crate_style;

import_crate_style!(css, "./css/main.module.css");

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router
            trailing_slash=leptos_router::TrailingSlash::Redirect
            base="leptos-gh-pages-test"
        >
            <nav class=css::nav>
                <A href="/">
                    <h3>{"Go Home"}</h3>
                </A>
            </nav>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/layouts" view=LayoutsWrapper>
                    <Route path=":name" view=Layout/>
                    <Route path="/" view=Layouts/>
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    "Home page"
}

#[component]
fn LayoutsWrapper() -> impl IntoView {
    view! {
        <h2>{"Here be layouts"}</h2>
        <Outlet/>
    }
}

#[component]
fn Layout() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with(|p| p.get("name").cloned().unwrap_or_default());
    
    view! {
        <h4>{name}</h4>
    }
}

#[component]
fn Layouts() -> impl IntoView {
    let layouts = [
        "layout1",
        "layout2",
        "layout3",
        "layout4",
        "layout5"
    ];

    let url = |s: &str| format!("/layout-files/{s}.txt");

    view! {
        <ul>{move || {
            layouts.into_iter()
                .map(|i| view! {
                    <li><A href=url(&i)>{i}</A></li>
                })
                .collect_view()
        }}
        </ul>
    }
}

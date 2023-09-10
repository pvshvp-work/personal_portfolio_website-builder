use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

// Generate all routes and output them to the dist path
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use dioxus_fullstack::prelude::pre_cache_static_routes_with_props;
    use dioxus_fullstack::prelude::IncrementalRendererConfig;
    use dioxus_fullstack::prelude::ServeConfigBuilder;
    use dioxus_fullstack::router::FullstackRouterConfig;

    let full_stack_router_config = FullstackRouterConfig::<Route>::default();
    let incremental_renderer_config = IncrementalRendererConfig::default().static_dir("dist");
    let serve_config = ServeConfigBuilder::new_with_router(full_stack_router_config)
        .assets_path("dist")
        .incremental(incremental_renderer_config)
        .build();

    pre_cache_static_routes_with_props(&serve_config)
        .await
        .unwrap();
}

// Hydrate the page
#[cfg(feature = "web")]
fn main() {
    use dioxus_fullstack::prelude::get_root_props_from_document;
    use dioxus_fullstack::router::RouteWithCfg;

    let root_component = RouteWithCfg::<Route>;
    let root_properties =
        get_root_props_from_document().expect("Failed to get root props from document");
    let web_config = dioxus_web::Config::default().hydrate(true);

    dioxus_web::launch_with_props(root_component, root_properties, web_config);
}

#[cfg(not(any(feature = "web", feature = "ssr")))]
fn main() {}

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog,
}

#[allow(non_snake_case)]
#[inline_props]
fn Blog(cx: Scope) -> Element {
    render! {
        Link { to: Route::Home {}, "Go to counter" }
        table {
            tbody {
                for _ in 0..100 {
                    tr {
                        for _ in 0..100 {
                            td { "hello world!" }
                        }
                    }
                }
            }
        }
    }
}

#[allow(non_snake_case)]
#[inline_props]
fn Home(cx: Scope<HomeProps>) -> Element {
    let mut count = use_state(cx, || 0);
    let text = use_state(cx, || "...".to_string());

    cx.render(rsx! {
        Link {
            to: Route::Blog {},
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count} {text}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    })
}

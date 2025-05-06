use gloo_net::http::Request;
use serde::Deserialize;
use skeleton_rs::yew::{Skeleton, SkeletonGroup};
use skeleton_rs::{Animation, Theme, Variant};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(Example1)]
pub fn example1() -> Html {
    html! { <Skeleton variant={Variant::Image} width="300px" height="200px" show=false /> }
}

#[function_component(Example2)]
pub fn example2() -> Html {
    html! {
        <Skeleton
            variant={Variant::Text}
            width="100%"
            height="1.5em"
            border_radius="0"
            animation={Animation::Wave}
            show=false
        />
    }
}

#[function_component(Example3)]
pub fn example3() -> Html {
    html! {
        <Skeleton
            variant={Variant::Avatar}
            width="80px"
            height="80px"
            theme={Theme::Dark}
            animation={Animation::Pulse}
            show=false
        />
    }
}

#[function_component(Example4)]
pub fn example4() -> Html {
    html! {
        <Skeleton
            variant={Variant::Button}
            width="150px"
            height="40px"
            animate_on_hover=true
            animate_on_active=true
            animation={Animation::Pulse}
            theme={Theme::Custom("#0099ff")}
            show=false
        />
    }
}

#[function_component(Example5)]
pub fn example5() -> Html {
    html! {
        <SkeletonGroup style="display: flex; gap: 1rem;">
            <Skeleton variant={Variant::Rounded} width="100px" height="100px" />
            <Skeleton variant={Variant::Rounded} width="100px" height="100px" />
            <Skeleton variant={Variant::Rounded} width="100px" height="100px" />
        </SkeletonGroup>
    }
}

#[function_component(Example6)]
pub fn example6() -> Html {
    html! {
        <Skeleton
            variant={Variant::Text}
            animation={Animation::Wave}
            animate_on_visible=true
            height="20px"
        />
    }
}

#[function_component(Example7)]
pub fn example7() -> Html {
    html! {
        <Skeleton
            variant={Variant::Image}
            width="100%"
            height="400px"
            responsive=true
            animate_on_focus=true
            animation={Animation::Wave}
        />
    }
}

#[function_component(Example8)]
pub fn example8() -> Html {
    html! {
        <Skeleton
            variant={Variant::Button}
            width="200px"
            height="50px"
            animation={Animation::Pulse}
            custom_style="box-shadow: 0 0 10px rgba(0,0,0,0.2);"
        />
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

#[function_component(Example9)]
pub fn example9() -> Html {
    let loading = use_state(|| false);
    let post = use_state(|| None);

    let fetch_post = {
        let loading = loading.clone();
        let post = post.clone();
        Callback::from(move |_| {
            let loading = loading.clone();
            let post = post.clone();
            spawn_local(async move {
                loading.set(true);
                let fetched: Post = Request::get("https://jsonplaceholder.typicode.com/posts/1")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                post.set(Some(fetched));
                loading.set(false);
            });
        })
    };

    html! {
        <div class="space-y-4">
            <button onclick={fetch_post.clone()}>{ "Fetch Post" }</button>
            <SkeletonGroup>
                <Skeleton variant={Variant::Text} height="1.5em" width="100%" show={!*loading}>
                    if let Some(p) = &*post {
                        <h2>{ p.title.clone() }</h2>
                    }
                </Skeleton>
                <Skeleton variant={Variant::Text} height="5em" width="100%" show={!*loading}>
                    if let Some(p) = &*post {
                        <p>{ p.body.clone() }</p>
                    }
                </Skeleton>
            </SkeletonGroup>
        </div>
    }
}

#[function_component(Example10)]
pub fn example10() -> Html {
    html! {
        <Skeleton variant={Variant::Rounded} width="100%" height="250px" delay_ms=1000 show=false />
    }
}

#[function_component(Example11)]
pub fn example11() -> Html {
    html! {
        <Skeleton
            variant={Variant::Image}
            width="100%"
            height="300px"
            animation={Animation::Wave}
            min_height="100px"
            max_height="400px"
            custom_style="border: 2px dashed red;"
        />
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Skeleton RS Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                { (1..=11).map(|i| {
                        let (title, component, code) = match i {
                            1 => ("Basic Skeleton", html! { <Example1 /> }, r#"use yew::prelude::*;
use skeleton_rs::yew::Skeleton;
use skeleton_rs::Variant;

#[function_component(Example1)]
pub fn example1() -> Html {
    html! {
        <Skeleton variant={Variant::Image} width="300px" height="200px" show={false} />
    }
}"#),
                            2 => ("Wave Text", html! { <Example2 /> }, r#"use yew::prelude::*;
use crate::components::skeleton::{Skeleton, Variant, Animation};

#[function_component(Example2)]
pub fn example2() -> Html {
    html! {
        <Skeleton
            variant={Variant::Text}
            width="100%"
            height="1.5em"
            border_radius="0"
            animation={Animation::Wave}
            show={false}
        />
    }
}"#),
                            3 => ("Dark Avatar", html! { <Example3 /> }, r#"use yew::prelude::*;
use skeleton_rs::yew::Skeleton;
use skeleton_rs::{Theme, Variant};

#[function_component(Example3)]
pub fn example3() -> Html {
    html! {
        <Skeleton
            variant={Variant::Avatar}
            width="80px"
            height="80px"
            theme={Theme::Dark}
            animation={Animation::Pulse}
            show={false}
        />
    }
}"#),
                            4 => ("Button Skeleton", html! { <Example4 /> }, r#"use yew::prelude::*;
use skeleton_rs::yew::Skeleton;
use skeleton_rs::{Animation, Theme, Variant};

#[function_component(Example4)]
pub fn example4() -> Html {
    html! {
        <Skeleton
            variant={Variant::Button}
            width="150px"
            height="40px"
            animate_on_hover={true}
            animate_on_active={true}
            animation={Animation::Pulse}
            theme={Theme::Custom("\#0099ff")}
            show={false}
        />
    }
}"#),
                            5 => ("Skeleton Group", html! { <Example5 /> }, r#"use yew::prelude::*;
use skeleton_rs::yew::{Skeleton, SkeletonGroup};
use skeleton_rs::Variant;

#[function_component(Example5)]
pub fn example5() -> Html {
    html! {
        <SkeletonGroup style="display: flex; gap: 1rem;">
            <Skeleton variant={Variant::Rounded} width="100px" height="100px" />
            <Skeleton variant={Variant::Rounded} width="100px" height="100px" />
            <Skeleton variant={Variant::Rounded} width="100px" height="100px" />
        </SkeletonGroup>
    }
}"#),
                            6 => ("Animate on Visible", html! { <Example6 /> }, r#"use yew::prelude::*;
use skeleton_rs::yew::Skeleton;
use skeleton_rs::{Animation, Variant};

#[function_component(Example6)]
pub fn example6() -> Html {
    html! {
        <Skeleton
            variant={Variant::Text}
            animation={Animation::Wave}
            infer_size={true}
            animate_on_visible={true}
            height="20px"
        />
    }
}"#),
                            7 => ("Responsive Image Skeleton", html! { <Example7 /> }, r#"use yew::prelude::*;
use skeleton_rs::yew::Skeleton;
use skeleton_rs::{Animation, Variant};

#[function_component(Example7)]
pub fn example7() -> Html {
    html! {
        <Skeleton
            variant={Variant::Image}
            width="100%"
            height="400px"
            responsive={true}
            animate_on_focus={true}
            animation={Animation::Wave}
        />
    }
}"#),
                            8 => ("Styled Button", html! { <Example8 /> }, r#"use yew::prelude::*;
use skeleton_rs::yew::Skeleton;
use skeleton_rs::{Animation, Variant};

#[function_component(Example8)]
pub fn example8() -> Html {
    html! {
        <Skeleton
            variant={Variant::Button}
            width="200px"
            height="50px"
            animation={Animation::Pulse}
            custom_style="box-shadow: 0 0 10px rgba(0,0,0,0.2);"
        />
    }
}"#),
                            9 => ("Fetch Data Example", html! { <Example9 /> }, r#"use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use skeleton_rs::yew::{Skeleton, SkeletonGroup};
use skeleton_rs::Variant;

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

#[function_component(Example9)]
pub fn example9() -> Html {
    let loading = use_state(|| false);
    let post = use_state(|| None);

    let fetch_post = {
        let loading = loading.clone();
        let post = post.clone();
        Callback::from(move |_| {
            let loading = loading.clone();
            let post = post.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let fetched: Post = Request::get("https://jsonplaceholder.typicode.com/posts/1")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                post.set(Some(fetched));
                loading.set(false);
            });
        })
    };

    html! {
        <div class="space-y-4">
            <button onclick={fetch_post.clone()}>{ "Fetch Post" }</button>
            <SkeletonGroup>
                <Skeleton
                    variant={Variant::Text}
                    height="1.5em"
                    width="100%"
                    show={!*loading}
                >
                if let Some(p) = &*post {
                    <h2>{ p.title.clone() }</h2>
                } 
                </Skeleton>
                <Skeleton
                    variant={Variant::Text}
                    height="5em"
                    width="100%"
                    show={!*loading}
                >
                if let Some(p) = &*post {
                    <p>{ p.body.clone() }</p>
                } 
                </Skeleton>
            </SkeletonGroup>
        </div>
    }
}"#),
                            10 => ("Delayed Skeleton", html! { <Example10 /> }, r#"use yew::prelude::*;
use skeleton_rs::yew::Skeleton;
use skeleton_rs::Variant;

#[function_component(Example10)]
pub fn example10() -> Html {
    html! {
        <Skeleton
            variant={Variant::Rounded}
            width="100%"
            height="250px"
            delay_ms={1000}
            show={false}
        />
    }
}"#),
                            11 => ("Custom Style Skeleton", html! { <Example11 /> }, r#"use yew::prelude::*;
use skeleton_rs::yew::Skeleton;
use skeleton_rs::{Animation, Variant};

#[function_component(Example11)]
pub fn example11() -> Html {
    html! {
        <Skeleton
            variant={Variant::Image}
            width="100%"
            height="300px"
            animation={Animation::Wave}
            min_height="100px"
            max_height="400px"
            custom_style="border: 2px dashed red;"
        />
    }
}"#),
                            _ => unreachable!()
                        };

                        html! {
                            <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                                <h2 class="text-xl font-bold mb-2">{ title }</h2>
                                <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                                    { code }
                                </pre>
                                { component }
                            </div>
                        }
                    }).collect::<Html>() }
            </div>
        </div>
    }
}

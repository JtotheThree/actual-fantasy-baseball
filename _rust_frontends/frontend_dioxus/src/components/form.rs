use dioxus::prelude::*;

/***** TITLE ******/
#[inline_props]
pub fn form_title<'a>(cx: Scope, text: &'a str) -> Element {
    cx.render(rsx!{
        h2 { class: "h-5 text-center border-b-2 border-gray-700 tracking-wid px-4 text-3xl font-title",
            span { class: "bg-paper px-8",
                "{text}"
            }
        }
    })
}


/***** INPUT ******/
#[derive(Props)]
pub struct FormInputProps<'a> {
    label: &'a str,
    r#type: &'a str,
    state: UseState<'a, String>,
}

pub fn form_input<'a>(cx: Scope<'a, FormInputProps<'a>>) -> Element<'a> {
    cx.render(rsx!{
        div { class: "flex flex-col py-3",
            input { class: "p-2 border-0 border-b-2 border-black outline-none bg-paper focus:ring focus:ring-red-700",
                r#type: "{cx.props.r#type}",
                oninput: move |evt| cx.props.state.set(evt.value.clone())
            }
            label { class: "px-4 pb-2 font-semibold",
                "{cx.props.label}"
            }
        }
    })
}

/***** SUBMIT ******/
#[inline_props]
pub fn form_submit<'a>(cx: Scope, name: &'a str) -> Element<'a> {
    cx.render(rsx!{
        div { class: "mt-2",
            button { class: "btn p-3 my-2 bg-gray-700 text-paper rounded-sm border-b-4 border-paper w-full font-bold hover:bg-red-800",
                "{name}"
            }
        }
    })
}
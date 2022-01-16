use dioxus::{
    prelude::*,
    router::*,
};

pub fn header(cx: Scope) -> Element {
    cx.render(rsx! {
        header { class: "border-solid border-b-2 border-black shadow-md",
            nav { class: "flex flex-wrap items-center justify-between w-full py-4 md:py-0 px-4 text-2xl bg-paper",
                ul { class: "flex flex-wrap",
                    Link {
                        class: "font-title",
                        to: "/",
                        "Actual Fantasy Baseball"
                    }
                }
                ul { class: "pt-4 text-base md:flex md:justify-between md:pt-0 font-normal font-bold",
                    Link {
                        class: "md:p-4 py-2 block hover:text-red-800",
                        to: "/login",
                        "Login"
                    }
                }
            }
        }
    })
}
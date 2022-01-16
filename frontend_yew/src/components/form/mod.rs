use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormTitleProps {
    pub title: String
}

#[function_component(FormTitle)]
pub fn form_title(props: &FormTitleProps) -> Html {
    html! {
        <h2 class="h-5 text-center border-b-2 border-gray-700 tracking-wid px-4 text-3xl font-title">
            <span class="bg-paper px-8">
                { props.title.clone() }
            </span>
        </h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormInputProps {
    #[prop_or(String::from("input"))]
    pub _type: String,
    pub value: NodeRef,
    pub label: String,
}

#[function_component(FormInput)]
pub fn form_input(props: &FormInputProps) -> Html {
    html! {
        <div class="flex flex-col py-3">
            <input
                class="input"
                type={props._type.clone()}
                ref={props.value.clone()}
            />
            <label class="px-4 pb-2 font-semibold">
                { props.label.clone() }
            </label>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormCheckboxProps {
    pub value: NodeRef,
    pub label: String,
}

#[function_component(FormCheckbox)]
pub fn form_checkbox(props: &FormCheckboxProps) -> Html {
    html! {
        <div class="flex flex-col py-3">
            <input
                class="form-checkbox h-5 w-5 text-red-700"
                type="checkbox"
                ref={props.value.clone()}
            />
            <label class="px-4 pb-2 font-semibold">
                { props.label.clone() }
            </label>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormNumberProps {
    pub value: NodeRef,
    pub min: String,
    pub max: String,
    pub default: String,
    pub label: String,
}

#[function_component(FormNumber)]
pub fn form_number(props: &FormNumberProps) -> Html {
    html! {
        <div class="flex flex-col py-3">
            <input
                class="input"
                type="number"
                min={props.min.clone()}
                max={props.max.clone()}
                value={props.default.clone()}
                ref={props.value.clone()}
            />
            <label class="px-4 pb-2 font-semibold">
                { props.label.clone() }
            </label>
        </div>
    }
}


#[derive(Properties, PartialEq)]
pub struct FullPageFormProps {
    pub title: String,
    pub error: String,
    pub children: Children,
    pub submit_label: String,
    pub onsubmit: Callback<FocusEvent>,
}

#[function_component(FullPageForm)]
pub fn full_page_form(props: &FullPageFormProps) -> Html {
    let error = if props.error != "" {
        html! {
            <span class="text-red-800">{"Error: "}{props.error.to_string()}</span>
        }
    } else {
        info!{"No error"};
        html!{}
    };

    html! {
        <div class="w-full justify-self-center md:w-1/2 mx-auto flex flex-col flex-wrap justify-between p-16">
            <FormTitle title={props.title.clone()} />
            <form class="md:w-1/2-screen m-0 p-12 w-full tw-h-full shadow-md" onsubmit={props.onsubmit.clone()}>
                { error }
                { for props.children.iter() }

                <div class="mt-2">
                    <button
                        class="btn p-3 my-2 bg-gray-700 text-paper rounded-sm border-b-4 border-paper w-full font-bold hover:bg-red-800"
                        type="submit"
                        disabled=false>
                        { props.submit_label.clone() }
                    </button>
                </div>
            </form>
        </div>
    }
}
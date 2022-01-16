use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(Callback::noop())]
    /// Click event for dropdown item
    pub onclick: Callback<String>,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// General property to add returned data
    #[prop_or_default]
    pub data: String,
    pub text: String,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let onclick = {
        let props = props.clone();
        Callback::from(move |_| {
            props.onclick.emit(props.data.clone());
        })
    };

    html! {
        <button
            {onclick}
            class="align-self-end btn py-1 px-8 bg-gray-700 text-paper rounded-sm border-b-4 border-paper font-bold hover:bg-red-800 float-right">
            { props.text.clone() }
        </button>
    }
}
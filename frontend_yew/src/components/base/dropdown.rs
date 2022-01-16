use yew::prelude::*;

// DROPDOWN
#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    pub parent: Html,
    #[prop_or_default]
    pub key: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    html! {
        <li class={classes!(props.class.clone(), "group", "relative", "dropdown")}>
            { props.parent.clone() }
            <div class="group-hover:block dropdown-menu absolute hidden h-auto py-3 flex-grow">
                <ul class="top-0 w-128 bg-paper shadow border-2 border-black px-6 py-2">
                    { props.children.clone() }
                </ul>
            </div>
        </li>
    }
}


// DROPDOWN ITEM
#[derive(Properties, Clone, PartialEq)]
pub struct DropdownItemProps {
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
    pub children: Children,
}

#[function_component(DropdownItem)]
pub fn dropdownitem(props: &DropdownItemProps) -> Html {
    let data = props.data.clone();

    let class = classes!(props.class.clone(), String::from("text-left md:p-4 py-2 block font-bold hover:text-red-800"));

    html! {
        <li class="py-1">
            <button class={class} key={props.key.clone()} onclick={props.onclick.reform(move |_| data.clone())}>
                { props.children.clone() }
            </button>
        </li>
    }
}
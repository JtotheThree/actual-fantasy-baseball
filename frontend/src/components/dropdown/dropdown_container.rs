use yew::prelude::*;

pub struct Dropdown {
    props: Props,
    active: bool,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// clickeable content to show the dropdown. Required
    pub main_content: Html,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

impl Component for Dropdown {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        let class = self.props.class.clone() + " group relative dropdown";

        html! {
            <li class=class>
                <a href="#">
                    { self.props.main_content.clone() }
                </a>
                <div class="group-hover:block dropdown-menu absolute hidden h-auto py-3">
                    <ul class="top-0 w-128 bg-paper shadow border-2 border-black px-6 py-2">
                        { self.props.children.clone() }
                    </ul>
                </div>
            </li>
        }
    }
}


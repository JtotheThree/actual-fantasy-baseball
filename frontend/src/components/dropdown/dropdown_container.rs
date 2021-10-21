use yew::prelude::*;

pub struct Dropdown {
    props: Props,
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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        info!("re-viewing dropdown container");
        let class = self.props.class.clone() + " group relative dropdown";

        html! {
            <li class=class>
                { self.props.main_content.clone() }
                <div class="group-hover:block dropdown-menu absolute hidden h-auto py-3">
                    <ul class="top-0 w-128 bg-paper shadow border-2 border-black px-6 py-2">
                        { self.props.children.clone() }
                    </ul>
                </div>
            </li>
        }
    }
}


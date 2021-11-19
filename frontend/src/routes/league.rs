use yew::prelude::*;

pub struct League {
    props: Props,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub league_id: String,
}

impl Component for League {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        League {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="px-4">
            <div
              class="flex items-center justify-center max-w-2xl p-16 mx-auto my-16 border-2 border-black  font-raleway"
            >
            <h1>{ self.props.league_id.clone() }</h1>
            </div>
            </div>
        }
    }
}

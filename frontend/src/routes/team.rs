use yew::prelude::*;

pub struct Team {
    props: Props,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub team_id: String,
}

impl Component for Team {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Team {
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
              class="
                flex
                justify-center
                items-center
                mx-auto
                max-w-2xl
                my-16
                p-16
                border-2
                border-black
                font-raleway
              "
            >
               <h1>{ self.props.team_id.clone() }</h1>
            </div>
            </div>
        }
    }
}

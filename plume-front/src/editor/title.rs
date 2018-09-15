use stdweb::web::{document, INonElementParentNode, INode};
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Title {
    content: String,
}

pub enum Msg {
    Update,
}

impl Component for Title {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Title {
            content: String::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        ConsoleService::new().log("bloup:");
        match msg {
            Msg::Update => self.content = document()
                .get_element_by_id("title")
                .and_then(|t| t.text_content())
                .unwrap_or(String::new())
                // Remove the placeholder from the content
                .splitn(2, "Title").last().unwrap_or("").to_string(),
        }
        ConsoleService::new().log("content:");
        ConsoleService::new().log(self.content.as_ref());
        ConsoleService::new().log(format!("{}", self.content.is_empty()).as_ref());
        true
    }
}

impl Renderable<Title> for Title {
    fn view(&self) -> Html<Self> {
        let classes = if self.content.is_empty() {
            "placeholder show"
        } else {
            "placeholder"
        };

        html! {
            <h1 id="title", contenteditable=true, onkeyup=|_| Msg::Update, onblur=|_| Msg::Update,>
                <span contenteditable=false, class=classes,>{ "Title" }</span>
            </h1>
        }
    }
}

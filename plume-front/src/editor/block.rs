use rand::prelude::*;
use stdweb::web::{document, INonElementParentNode, INode};
use yew::prelude::*;
use yew::services::ConsoleService;

#[derive(Clone, Debug, PartialEq)]
pub enum BlockKind {
    Paragraph(String),
    Header(u32, String),
    Image(String),
    Video(String),
    Audio(String),
    Separator,
    Embed(String),
    Code(String),
    Quote(String, String),
}

impl Default for BlockKind {
    fn default() -> Self {
        BlockKind::Paragraph(String::new())
    }
}

#[derive(Debug)]
pub struct Block {
    kind: BlockKind,
    show_placeholder: bool,
    placeholder: &'static str,
    content: String,
    id: String,
    has_menu: bool,
    menu_opened: bool,
    has_focus: bool,
}

#[derive(Debug)]
pub enum Msg {
    Update,
    ToggleMenu,
    Focus,
    Leave,
    HeaderUp,
}

#[derive(Clone, Default, PartialEq)]
pub struct Props {
    pub kind: BlockKind,
    pub show_placeholder: bool,
    pub placeholder: Option<&'static str>,
    pub has_menu: bool,
}

impl Component for Block {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Block {
            kind: props.kind,
            show_placeholder: props.show_placeholder,
            placeholder: props.placeholder.unwrap_or("…"),
            content: String::new(),
            id: format!("block-{}", random::<u32>()),
            has_menu: props.has_menu,
            menu_opened: false,
            has_focus: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update => {
                let elt = document().get_element_by_id(self.id.as_ref());
                let new = elt.clone().and_then(|t| t.text_content())
                    .unwrap_or(String::new())
                    // Remove the placeholder from the content
                    .splitn(2, self.placeholder).last().unwrap_or("").to_string();
                self.kind = match self.kind {
                    BlockKind::Header(level, _) => BlockKind::Header(level, new.clone().chars().rev().collect()),
                    BlockKind::Paragraph(_) => BlockKind::Paragraph(new.clone().chars().rev().collect()),
                    ref x => x.clone()
                };
            },
            Msg::ToggleMenu => self.menu_opened = !self.menu_opened,
            Msg::Leave => self.has_focus = false,
            Msg::Focus => self.has_focus = true,
            Msg::HeaderUp => self.kind = match self.kind {
                BlockKind::Header(level, ref content) => BlockKind::Header(level - 1, content.clone()),
                BlockKind::Paragraph(ref content) => BlockKind::Header(2, content.clone()),
                _ => BlockKind::Header(2, String::new()),
            }
        }
        ConsoleService::new().log(format!("{:?} => {:?}", msg, self).as_ref());
        true
    }
}

impl<'a> Renderable<Block> for Block {
    fn view(&self) -> Html<Block> {
        let menu = if self.has_focus {
            "actions show"
        } else {
            "actions"
        };
        let placeholder = if self.show_placeholder && !self.has_focus && false {
            let classes = if self.content.is_empty() {
                "placeholder show"
            } else {
                "placeholder"
            };
            html! {<span class=classes, contenteditable=false,>{ self.placeholder }</span>}
        } else {
            html! {<></>}
        };

        let main = match self.kind {
            BlockKind::Paragraph(ref content) => html! {
                <p id=self.id.as_str(), contenteditable=true, onkeyup=|_| Msg::Update, onblur=|_| Msg::Update,>{ content.clone() }</p>
            },
            BlockKind::Header(level, ref content) => match level {
                1 => html! { <h1 id=self.id.as_str(), contenteditable=true, onkeyup=|_| Msg::Update, onblur=|_| Msg::Update,>{ placeholder }{ content.clone() }</h1>},
                2 => html! { <h2 id=self.id.as_str(), contenteditable=true, onkeyup=|_| Msg::Update, onblur=|_| Msg::Update,>{ placeholder }{ content.clone() }</h2>},
                3 => html! { <h3 id=self.id.as_str(), contenteditable=true, onkeyup=|_| Msg::Update, onblur=|_| Msg::Update,>{ placeholder }{ content.clone() }</h3>},
                4 => html! { <h4 id=self.id.as_str(), contenteditable=true, onkeyup=|_| Msg::Update, onblur=|_| Msg::Update,>{ placeholder }{ content.clone() }</h4>},
                5 => html! { <h5 id=self.id.as_str(), contenteditable=true, onkeyup=|_| Msg::Update, onblur=|_| Msg::Update,>{ placeholder }{ content.clone() }</h5>},
                _ => html! { <h6 id=self.id.as_str(), contenteditable=true, onkeyup=|_| Msg::Update, onblur=|_| Msg::Update,>{ placeholder }{ content.clone() }</h6>},
            },
            BlockKind::Audio(ref src) => html! {
                <audio src=src,></audio>
            },
            BlockKind::Code(_) => html! {
                <pre><code contenteditable=true,></code></pre>
            },
            BlockKind::Embed(ref src) => html! {
                <iframe src=src,></iframe>
            },
            BlockKind::Image(ref src) => html! {
                <img src=src, />
            },
            BlockKind::Quote(ref content, ref author) => html! {
                <blocquote contenteditable=true,></blocquote>
            },
            BlockKind::Separator => html! { <hr/> },
            BlockKind::Video(ref src) => html! {
                <video src=src,></video>
            },
        };

        html! {
            <div class="block-wrapper", onfocus=|_| Msg::Focus, onblur=|_| Msg::Leave,>
                {if self.has_menu {
                    html! {<div class=menu,>
                        <div class="action main", onclick=|_| Msg::ToggleMenu,><i class="icon icon-more-horizontal",></i></div>
                        {if self.menu_opened {
                            html! {
                                <div class="action", onclick=|_| Msg::HeaderUp,>
                                    <i class="icon icon-chevron-up",></i>
                                </div>
                            }
                        } else {
                            html! {<></>}
                        }}
                    </div>}
                } else {
                    html! {<></>}
                }}
                { main }
            </div>
        }
    }
}

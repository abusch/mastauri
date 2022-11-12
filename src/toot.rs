use mastodon_model::{Account, Status};
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq, Eq)]
pub struct TootListProps {
    pub toots: Vec<Status>,
}

#[function_component(TootList)]
pub fn toot_list(TootListProps { toots }: &TootListProps) -> Html {
    let list = toots
        .iter()
        .map(|toot| {
            html! {
                 <Toot toot={toot.clone()} />
            }
        })
        .collect::<Html>();

    html! {
        <div class="toots">
        {list}
        </div>
    }
}

#[derive(Clone, Properties, PartialEq, Eq)]
pub struct TootProps {
    pub toot: Status,
}

#[function_component(Toot)]
pub fn toot(TootProps { toot }: &TootProps) -> Html {
    html! {
        <div class="toot">
            <TootHeader account={toot.account.clone()} />
            <TootContent content={toot.content.clone()} />
        </div>
    }
}

#[derive(Clone, Properties, PartialEq, Eq)]
pub struct TootContentProps {
    content: String,
}

pub struct TootContent;

impl Component for TootContent {
    type Message = ();

    type Properties = TootContentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let div = gloo_utils::document().create_element("div").unwrap();
        div.set_inner_html(&ctx.props().content);
        div.set_class_name("toot-content");
        Html::VRef(div.into())
    }
}

#[derive(Clone, Properties, PartialEq, Eq)]
pub struct TootHeaderProps {
    account: Account,
}

#[function_component(TootHeader)]
pub fn toot_header(TootHeaderProps { account }: &TootHeaderProps) -> Html {
    html! {
        <div class="toot-header">
            <img class="avatar" width={48} height={48} src={account.avatar.to_string()} />
            <div class="account-info">
                <p class="display-name">{&account.display_name}</p>
                <p class="username">{&account.acct}</p>
            </div>
        </div>
    }
}

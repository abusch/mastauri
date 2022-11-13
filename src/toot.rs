use mastodon_model::Status;
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
            <TootHeader
                account_display_name={toot.resolved_account_display_name()}
                account_acct={toot.account.acct.clone()}
                avatar_url={toot.account.avatar.to_string()}
            />
            <SanitizedHtml content={toot.resolved_content()} class_name = "toot-content"/>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq, Eq)]
pub struct SanitizedHtmlProps {
    content: String,
    class_name: String,
}

pub struct SanitizedHtml;

impl Component for SanitizedHtml {
    type Message = ();

    type Properties = SanitizedHtmlProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let div = gloo_utils::document().create_element("div").unwrap();
        div.set_inner_html(&ctx.props().content);
        div.set_class_name(&ctx.props().class_name);
        Html::VRef(div.into())
    }
}

#[derive(Clone, Properties, PartialEq, Eq)]
pub struct TootHeaderProps {
    account_display_name: String,
    account_acct: String,
    avatar_url: String,
}

#[function_component(TootHeader)]
pub fn toot_header(
    TootHeaderProps {
        account_display_name,
        account_acct,
        avatar_url,
    }: &TootHeaderProps,
) -> Html {
    html! {
        <div class="toot-header">
            <img class="avatar" width={48} height={48} src={avatar_url.clone()} />
            <div class="account-info">
                <SanitizedHtml
                    content={account_display_name.clone()}
                    class_name="display-name"
                />
                <p class="username">{&account_acct}</p>
            </div>
        </div>
    }
}

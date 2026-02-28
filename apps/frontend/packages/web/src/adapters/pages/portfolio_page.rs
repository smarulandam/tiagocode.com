use dioxus::prelude::*;

use ui::PortfolioView;

#[component]
pub fn PortfolioPage(lang: String) -> Element {
    let data = use_loader(|| async move { api::portfolio_detail_controller().await })?;
    let portfolio = data.read().clone();

    rsx! {
        PortfolioView { portfolio: portfolio }
    }
}

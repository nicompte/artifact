/* artifact: the requirements tracking tool made for developers
 * Copyright (C) 2018  Garrett Berg <@vitiral, vitiral@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the Lesser GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the Lesser GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */

use yew_simple;
use stdweb::Value;
use dev_prelude::*;
use name;

pub(crate) fn view_nav(model: &Model, page: HtmlApp) -> HtmlApp {
    let search = &model.nav.search;
    let icon = if search.on {
        FA_SEARCH_MINUS
    } else {
        FA_SEARCH
    };
    let router = model.router.clone();
    html![<div>
        // Top Nav Bar (buttons)
        <div class=(CLEARFIX, MB2, ACE_WHITE, ACE_BG_BLACK, P1),>
            <button class=(BTN, REGULAR), id="search",
             onclick=|_| Msg::ToggleSearch,
             title="Search for an artifact.",>
                { fa_icon(icon) }
                <span class=ML1,>{ "Search" }</span>
            </button>

            <button class=(BTN, REGULAR), id="sync",
             onclick=|_| Msg::FetchProject,
             title="Sync frontend with file system.",>
                { fa_icon(FA_SYNC) }
                <span class=ML1,>{ "Sync" }</span>
            </button>

            <button class=(BTN, REGULAR), id="graph",
             onclick=move |_| {
                router.push_hash(Some("graph"))
             },
             title="View Graph",
             href="#graph",
             >
                { fa_icon(FA_GRAPH) }
                <span class=ML1,>{ "Graph" }</span>
            </button>

            <button class=(BTN, REGULAR), id="TEST",
             onclick=move |_| {
                Msg::PushLogs(vec![
                    Log::error(format!(
                        "<span>Created error at: {}</span>",
                        ::stdweb::web::Date::now(),
                    )),
                ])
             },
             title="TESTING",
             >
                <span>{ "TEST" }</span>
            </button>

        </div>

        // Embed the pages
        <div class=(CLEARFIX, PY1),>
            { error_pane(model) }
            { search_pane(model) }
            <div class=(SM_COL, SM_COL_11, MD_COL_7, LG_COL_9),>
                { page }
            </div>
        </div>
    </div>]
}

fn error_pane(model: &Model) -> HtmlApp {
    if model.logs.error.is_empty() {
        return html![<span></span>];
    }

    fn error_log(id: usize, log: &Log) -> HtmlApp {
        let msg = match Node::from_html(&log.html) {
            Ok(node) => VNode::VRef(node),
            Err(err) => html![
                <b>{ format!(
                    "INTERNALERROR: invalid html: {:?}\nERROR:{:?}",
                    log.html,
                    err)
                }</b>
            ],
        };
        html![
            <button class=(BTN, REGULAR),
             id=format!("close-err-{}", id),
             onclick=move |_| Msg::ClearLogs(ClearLogs::Error(vec![id])),
             title="clear error",
            >
                { fa_icon(FA_TIMES) }
                <span class=(ML1, ACE_RED),>
                    { msg }
                </span>
            </button>
        ]
    }

    html![<div class=(SM_COL, SM_COL_6, MD_COL_4, LG_COL_2, MR1),>
        <div class=H3,>
            <button class=BTN,
             id="close-err-all",
             onclick=|_| Msg::ClearLogs(ClearLogs::ErrorAll),
            >
                { fa_icon(FA_TIMES) }
                <span class=(ML1, ACE_RED),>
                    { "Clear All Errors" }
                </span>
            </button>
        </div>
        { for model.logs.error.iter().map(|(id, log)| error_log(*id, log)) }
    </div>]
}

fn search_pane(model: &Model) -> HtmlApp {
    if !model.nav.search.on {
        return html![<div></div>]
    }

    let names = match parse_regex(&model.nav.search.value) {
        Ok(re) => html![<div>
            { for model.shared.artifacts
                .keys()
                .filter(|n| re.is_match(n.as_str()))
                .map(|n| name::name_html(model, n))
            }
        </div>],
        Err(err) => err,
    };

    html![<div class=(SM_COL, SM_COL_6, MD_COL_4, LG_COL_2, MR1),>
        <input
         id="search-input",
         value=model.nav.search.value.clone(),
         oninput=|e: InputData| Msg::SetNavSearch(e.value),
         class=INPUT,
         ></input>
        { names }
    </div>]
}
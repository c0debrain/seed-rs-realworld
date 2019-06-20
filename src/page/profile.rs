use seed::prelude::*;
use super::{ViewPage, InitPage};
use crate::{session, username};

// Model

pub struct Model<'a> {
    session: session::Session<'a>
}

impl<'a> Model<'a> {
    pub fn session(&self) -> &session::Session {
        &self.session
    }
}

impl<'a> From<Model<'a>> for session::Session<'a> {
    fn from(model: Model<'a>) -> session::Session<'a> {
        model.session
    }
}

pub fn init<'a>(session: session::Session<'a>, username: &username::Username) -> InitPage<Model<'a>, Msg> {
    InitPage::new(Model { session })
}

// Update

pub enum Msg {

}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {

}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("@TODO", view_content())
}

fn view_content() -> El<Msg> {
    div![
        class!["profile-page"],

        div![
            class!["user-info"],
            div![
                class!["container"],
                div![
                    class!["row"],

                    div![
                        class!["col-xs-12", "col-md-10", "offset-md-1"],
                        img![
                            class!["user-img"],
                            attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                        ],
                        p![
                            "Cofounder @GoThinkster, lived in Aol's HQ for a few months, kinda looks like Peeta from the Hunger Games"
                        ],
                        button![
                            class!["btn", "btn-sm", "btn-outline-secondary", "action-btn"],
                            i![
                                class!["ion-plus-round"]
                            ],
                            raw!("&nbsp;"),
                            "Follow Eric Simons"
                        ]
                    ]

                ]
            ]
        ],

        div![
            class!["container"],
            div![
                class!["row"],
                div![
                    class!["col-xs-12", "col-md-10", "offset-md-1"],
                    div![
                        class!["articles-toggle"],
                        ul![
                            class!["nav", "nav-pills", "outline-active"],
                            li![
                                class!["nav-item"],
                                a![
                                    class!["nav-link", "active"],
                                    attrs!{At::Href => ""},
                                    "My Articles"
                                ]
                            ],
                            li![
                                class!["nav-item"],
                                a![
                                    class!["nav-link"],
                                    attrs!{At::Href => ""},
                                    "Favorited Articles"
                                ]
                            ]
                        ]
                    ],

                    div![
                        class!["article-preview"],
                        div![
                            class!["article-meta"],
                            a![
                                attrs!{At::Href => "/profile"},
                                img![
                                    attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                                ]
                            ],
                            div![
                                class!["info"],
                                a![
                                    class!["author"],
                                    attrs!{At::Href => ""},
                                    "Eric Simons"
                                ],
                                span![
                                    class!["date"],
                                    "January 20th"
                                ]
                            ],
                            button![
                                class!["btn","btn-outline-primary", "btn-sm", "pull-xs-right"],
                                i![
                                    class!["ion-heart"],
                                    " 29"
                                ]
                            ]
                        ],
                        a![
                            class!["preview-link"],
                            attrs!{At::Href => ""},
                            h1![
                                "How to build webapps that scale"
                            ],
                            p![
                                "This is the description for the post."
                            ],
                            span![
                                "Read more..."
                            ]
                        ]
                    ],

                    div![
                        class!["article-preview"],
                        div![
                            class!["article-meta"],
                            a![
                                attrs!{At::Href => "/profile"},
                                img![
                                    attrs!{At::Src => "http://i.imgur.com/N4VcUeJ.jpg"}
                                ]
                            ],
                            div![
                                class!["info"],
                                a![
                                    class!["author"],
                                    attrs!{At::Href => ""},
                                    "Albert Pai"
                                ],
                                span![
                                    class!["date"],
                                    "January 20th"
                                ]
                            ],
                            button![
                                class!["btn","btn-outline-primary", "btn-sm", "pull-xs-right"],
                                i![
                                    class!["ion-heart"],
                                    " 32"
                                ]
                            ]
                        ],
                        a![
                            class!["preview-link"],
                            attrs!{At::Href => ""},
                            h1![
                                "The song you won't ever stop singing. No matter how hard you try."
                            ],
                            p![
                                "This is the description for the post."
                            ],
                            span![
                                "Read more..."
                            ]
                        ]
                    ]

                ]
            ]
        ]

    ]
}
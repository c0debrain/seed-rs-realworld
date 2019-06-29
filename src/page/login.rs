use seed::{prelude::*, fetch};
use super::ViewPage;
use crate::{session, route, viewer, api, avatar, username, GMsg, form::login as form, login_fetch};
use serde::{Deserialize, Serialize};
use serde_json;
use std::rc::Rc;

// Model

#[derive(Default)]
pub struct Model {
    session: session::Session,
    problems: Vec<form::Problem>,
    form: form::Form,
}

impl Model {
    pub fn session(&self) -> &session::Session {
        &self.session
    }
}

impl From<Model> for session::Session {
    fn from(model: Model) -> session::Session {
        model.session
    }
}

// Init

pub fn init(session: session::Session, _: &mut impl Orders<Msg, GMsg>) -> Model {
    Model {
        session,
        ..Model::default()
    }
}

// Global msg handler

pub fn g_msg_handler(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session) => {
            model.session = session;
            route::go_to(route::Route::Home, orders);
        }
        _ => ()
    }
}

// Update

pub enum Msg {
    SubmittedForm,
    FieldChanged(form::Field),
    CompletedLogin(Result<viewer::Viewer, Vec<form::Problem>>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::SubmittedForm => {
            match model.form.trim_fields().validate() {
                Ok(valid_form) => {
                    model.problems.clear();
                    orders.perform_cmd(login_fetch::login(&valid_form, Msg::CompletedLogin));
                },
                Err(problems) => {
                    model.problems = problems;
                }
            }
        },
        Msg::FieldChanged(field) => {
            model.form.upsert_field(field);
        }
        Msg::CompletedLogin(Ok(viewer)) => {
            viewer.store();
            orders.send_g_msg(GMsg::SessionChanged(Some(viewer).into()));
        },
        Msg::CompletedLogin(Err(problems)) => {
            model.problems = problems;
        },
    }
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Login", view_content(model))
}

fn view_fieldset(field: &form::Field) -> El<Msg> {
    match field {
        form::Field::Email(value) => {
            fieldset![
                class!["form-group"],
                input![
                    class!["form-control", "form-control-lg"],
                    attrs!{
                        At::Type => "text",
                        At::Placeholder => "Email",
                        At::Value => value
                    },
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Email(new_value)
                    )),
                ]
            ]
        }
        form::Field::Password(value) => {
            fieldset![
                class!["form-group"],
                input![
                    class!["form-control", "form-control-lg"],
                    attrs!{
                        At::Type => "password",
                        At::Placeholder => "Password",
                        At::Value => value
                    },
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Password(new_value)
                    )),
                ]
            ]
        }
    }
}

fn view_form(form: &form::Form) -> El<Msg> {
    form![
        raw_ev(Ev::Submit, |event| {
            event.prevent_default();
            Msg::SubmittedForm
        }),
        form.iter().map(view_fieldset),
        button![
            class!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
            "Sign in"
        ]
    ]
}

fn view_content<'a>(model: &Model) -> El<Msg> {
    div![
        class!["auth-page"],
        div![
            class!["container", "page"],
            div![
                class!["row"],

                div![
                    class!["col-md-6", "offset-md-3", "col-x32-12"],
                    h1![
                        class!["text-xs-center"],
                        "Sign in"
                    ],
                    p![
                        class!["text-xs-center"],
                        a![
                            attrs!{At::Href => route::Route::Register.to_string()},
                            "Need an account?"
                        ]
                    ],

                    ul![
                        class!["error-messages"],
                        model.problems.iter().map(|problem| li![
                            problem.message()
                        ])
                    ],

                    view_form(&model.form)
                ]

            ]
        ]
    ]
}
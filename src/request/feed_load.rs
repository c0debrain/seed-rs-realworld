use serde::Deserialize;
use crate::{viewer, avatar, username, api, session, article, page, paginated_list};
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;
use std::rc::Rc;

const ARTICLES_PER_PAGE: usize = 5;

#[derive(Deserialize)]
struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    articles: Vec<ServerDataItemArticle>,
    articles_count: usize
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerDataItemArticle {
    title: String,
    slug: String,
    body: String,
    created_at: String,
    updated_at: String,
    tag_list: Vec<String>,
    description: String,
    author: ServerDataFieldAuthor,
    favorited: bool,
    favorites_count: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerDataFieldAuthor {
    username: String,
    bio: String,
    image: String,
    following: bool,
}

impl ServerData {
    fn into_paginated_list(self) -> paginated_list::PaginatedList<article::Article> {
        paginated_list::PaginatedList {
            values: self.articles.into_iter().map(|item| {
                article::Article {
                    title: item.title,
                    slug: item.slug.into(),
                    body: item.body,
                    created_at: item.created_at,
                    updated_at: item.updated_at,
                    tag_list: item.tag_list,
                    description: item.description,
                    author: article::Author {
                        username: item.author.username.into(),
                        bio: item.author.bio,
                        image: item.author.image,
                        following: item.author.following,
                    },
                    favorited: item.favorited,
                    favorites_count: item.favorites_count,
                }
            }).collect(),
            total: self.articles_count
        }
    }
}

pub fn request_url(
    username: &username::Username<'static>,
    feed_tab: &page::profile::FeedTab,
    page_number: &page::profile::PageNumber,
) -> String {
    format!(
        "https://conduit.productionready.io/api/articles?{}={}&limit={}&offset={}",
        match feed_tab {
            page::profile::FeedTab::MyArticles => "author",
            page::profile::FeedTab::FavoritedArticles => "favorited",
        },
        username.as_str(),
        ARTICLES_PER_PAGE,
        (page_number.to_usize() - 1) * ARTICLES_PER_PAGE
    )
}

pub fn load_feed<Ms: 'static>(
    session: session::Session,
    username: username::Username<'static>,
    feed_tab: page::profile::FeedTab,
    page_number: page::profile::PageNumber,
    f: fn(Result<paginated_list::PaginatedList<article::Article>, (username::Username<'static>, Vec<String>)>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {

    let username = username.clone();

    let mut request = fetch::Request::new(
        request_url(&username, &feed_tab, &page_number)
    ).timeout(5000);

    if let Some(viewer) = session.viewer() {
        let auth_token = viewer.credentials.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }

    request.fetch_string(move |fetch_object| {
        f(process_fetch_object(username, fetch_object))
    })
}

fn process_fetch_object(
    username: username::Username<'static>,
    fetch_object: fetch::FetchObject<String>
) -> Result<paginated_list::PaginatedList<article::Article>, (username::Username<'static>, Vec<String>)> {
    match fetch_object.result {
        Err(request_error) => {
            Err((username, vec!["Request error".into()]))
        },
        Ok(response) => {
            if response.status.is_ok() {
                    let paginated_list =
                        response
                            .data
                            .and_then(|string| {
                                serde_json::from_str::<ServerData>(string.as_str())
                                    .map_err(|error| {
                                        fetch::DataError::SerdeError(Rc::new(error))
                                    })
                            })
                            .map(|server_data| {
                                server_data.into_paginated_list()
                            });

                    match paginated_list {
                        Ok(paginated_list) => {
                            Ok(paginated_list)
                        },
                        Err(data_error) => {
                            Err((username, vec!["Data error".into()]))
                        }
                    }
            } else {
                let error_messages: Result<Vec<String>, fetch::DataError> =
                    response
                        .data
                        .and_then(|string| {
                            serde_json::from_str::<ServerErrorData>(string.as_str())
                                .map_err(|error| {
                                    fetch::DataError::SerdeError(Rc::new(error))
                                })
                        }).and_then(|server_error_data| {
                        Ok(server_error_data.errors.into_iter().map(|(field, errors)| {
                            format!("{} {}", field, errors.join(", "))
                        }).collect())
                    });
                match error_messages {
                    Ok(error_messages) => {
                        Err((username, error_messages))
                    },
                    Err(data_error) => {
                        Err((username, vec!["Data error".into()]))
                    }
                }
            }
        }
    }
}
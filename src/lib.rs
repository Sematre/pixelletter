use anyhow::{bail, Context};
use api::{error_code_to_msg, ActionType, Command, Options, Order};
use bon::{bon, Builder};
use isocountry::CountryCode;
use reqwest::multipart::{Form, Part};

use crate::api::{Addoption, Auth, Location};

pub mod api;

const BASE_URL: &str = "https://www.pixelletter.de/xml/index.php";
const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;

pub struct Client {
    client: reqwest::Client,
    auth: Auth,
}

#[bon]
impl Client {
    #[builder]
    pub fn new(
        client: Option<reqwest::Client>,
        email: String,
        password: String,
        agb: bool,
        widerrufsverzicht: bool,
        testing_mode: Option<bool>,
    ) -> Self {
        Client {
            client: client.unwrap_or_default(),
            auth: Auth {
                email,
                password,
                agb,
                widerrufsverzicht,
                testmodus: testing_mode.unwrap_or(false),
                auth_ref: None,
            },
        }
    }

    #[builder(finish_fn = submit)]
    pub async fn order(
        &self,
        letter: Option<Letter>,
        fax: Option<String>,
        files: Option<Vec<Part>>,
        text: Option<Text>,
        transaction: Option<String>,
    ) -> anyhow::Result<String> {
        if letter.is_none() && fax.is_none() {
            bail!("Neither `letter` nor `fax` are set!");
        }

        if !(files.is_none() ^ text.is_none()) {
            bail!("Set either `files` or `text`!");
        }

        if files.as_ref().map_or(false, Vec::is_empty) {
            bail!("`files` is empty!");
        }

        let letter = crate::api::Pixelletter {
            version: "1.3".to_owned(),

            auth: Some(self.auth.clone()),

            command: Some(Command {
                order: Some(Order {
                    content_type: match (files.as_ref(), text.as_ref()) {
                        (Some(_), None) => "upload",
                        (None, Some(_)) => "text",
                        (Some(_), Some(_)) | (None, None) => unreachable!(),
                    }
                    .to_owned(),

                    options: Options {
                        action: match (letter.as_ref(), fax.as_ref()) {
                            (Some(_), None) => ActionType::Letter,
                            (None, Some(_)) => ActionType::Fax,
                            (Some(_), Some(_)) => ActionType::LetterAndFax,
                            (None, None) => unreachable!(),
                        },

                        transaction,
                        control: "".to_owned(), // TODO: Implement control options
                        fax,
                        location: letter.as_ref().and_then(|lttr| lttr.location.to_owned()),

                        destination: letter
                            .as_ref()
                            .map(|lttr| lttr.destination.alpha2().to_owned()),

                        addoption: letter
                            .as_ref()
                            .and_then(|lttr| lttr.services.to_owned())
                            .unwrap_or_default(),

                        font: text.as_ref().map(|txt| txt.font.to_owned()),

                        returnaddress: text
                            .as_ref()
                            .map(|txt| txt.return_address.to_owned())
                            .unwrap_or_default(),
                    },

                    text: text.map(|txt| api::Text {
                        address: txt.address,
                        message: txt.message,
                    }),
                }),

                info: None,
                id: None,
            }),

            response: None,
            costumer_id: None,
            costumer_data: None,
            costumer_credit: None,
        };

        let mut form = Form::new().text(
            "xml",
            format!("{XML_HEADER}{}", quick_xml::se::to_string(&letter)?),
        );

        // Attach files to upload
        for (index, attachment) in files.into_iter().flatten().enumerate() {
            form = form.part(format!("uploadfile{index}"), attachment);
        }

        let resp_text = self
            .client
            .post(BASE_URL)
            .multipart(form)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let resp = quick_xml::de::from_str::<crate::api::Pixelletter>(&resp_text)?
            .response
            .context("No `response` field")?;

        println!("{resp:#?}");

        if resp.result.code == 100 {
            Ok(resp.result.msg)
        } else {
            bail!("{}", error_code_to_msg(resp.result.code as u8).unwrap())
        }
    }
}

#[derive(Builder)]
pub struct Letter {
    destination: CountryCode,
    location: Option<Location>,
    services: Option<Vec<Addoption>>,
}

#[derive(Builder)]
pub struct Text {
    address: String,
    message: String,
    font: String,
    return_address: String,
}

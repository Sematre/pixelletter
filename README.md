## Send registered mail using PDF
```rust
use pixelletter::{api::Addoption, Client, Letter};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder()
        .email("manfred@mueller.de".to_owned())
        .password("sa8Lioi".to_owned())
        .agb(true)
        .widerrufsverzicht(true)
        .testing_mode(true) // optional
        .build();

    let letter = Letter::builder()
        .destination(isocountry::CountryCode::DEU)
        .services(vec![Addoption::Einschreiben]) // optional
        .build();

    let file: Vec<u8> = std::fs::read("/path/to/letter.pdf")?;
    let file_part = reqwest::multipart::Part::bytes(file)
        .file_name("document.pdf")
        .mime_str("application/pdf")?;

    let resp = client
        .order()
        .letter(letter)
        .files(vec![file_part])
        .transaction("1234567890".to_string()) // optional
        .submit()
        .await?;

    println!("{resp}");
    Ok(())
}
```

## Send fax using raw text
```rust
use pixelletter::{Client, Text};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder()
        .email("manfred@mueller.de".to_owned())
        .password("sa8Lioi".to_owned())
        .agb(true)
        .widerrufsverzicht(true)
        .testing_mode(true) // optional
        .build();

    let text = Text::builder()
        .return_address("Lorem ipsum dolor sit amet.".to_owned())
        .address("Mr. John Doe\nAcme Corp.\n123 Glennwood Ave\nQuarto Creek, VA 22438".to_owned())
        .message("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do.".to_owned())
        .font("Arial".to_owned())
        .build();

    let resp = client
        .order()
        .fax("+49 152 28817386".to_owned())
        .text(text)
        .transaction("1234567890".to_string()) // optional
        .submit()
        .await?;

    println!("{resp}");
    Ok(())
}
```

#[macro_use]
extern crate serde_json;

use serde::Serialize;
use surf::Exception;

#[derive(Serialize)]
struct MyGetParams {
    a: u64,
    b: String,
}

fn test_reqwest() -> Result<(), Exception> {
    println!("> reqwest ...");

    let client = reqwest::Client::new();

    let mut res = client
        .get("https://blog.x5ff.xyz/other/cookbook2018")
        .send()?;

    assert_eq!(200, res.status());
    assert_eq!("Rust is awesome\n", res.text()?);

    let form_values = vec![
        ("custname", "Rusty Crabbington"),
        ("comments", "Thank you"),
        ("custemail", "rusty@nope.com"),
        ("custtel", "+1 234 33456"),
        ("delivery", "25th floor below ground, no elevator. sorry"),
    ];

    let res_forms: serde_json::Value = client
        .post("https://httpbin.org/post")
        .form(&form_values)
        .send()?
        .json()?;

    for (name, value) in form_values.iter() {
        assert_eq!(res_forms["form"][name], *value);
    }

    let json_payload = json!({
        "book": "Rust 2018 Cookbook",
        "blog": "https://blog.x5ff.xyz",
    });

    let res_json: serde_json::Value = client
        .put("https://httpbin.org/anything")
        .json(&json_payload)
        .send()?
        .json()?;

    assert_eq!(res_json["json"], json_payload);

    let query_params = MyGetParams {
        a: 0x5ff,
        b: "https://blog.x5ff.xyz".into(),
    };

    let res_query: serde_json::Value = client
        .get("https://httpbin.org/get")
        .query(&query_params)
        .send()?
        .json()?;

    assert_eq!(res_query["args"]["a"], query_params.a.to_string());
    assert_eq!(res_query["args"]["b"], query_params.b);

    println!("> reqwest successful!");
    Ok(())
}

async fn test_surf() -> Result<(), Exception> {
    println!("> surf ...");

    let client = surf::Client::new();
    let mut res = client
        .get("https://blog.x5ff.xyz/other/cookbook2018")
        .await?;

    assert_eq!(200, res.status());
    assert_eq!("Rust is awesome\n", res.body_string().await?);

    let form_values = vec![
        ("custname", "Rusty Crabbington"),
        ("comments", "Thank you"),
        ("custemail", "rusty@nope.com"),
        ("custtel", "+1 234 33456"),
        ("delivery", "25th floor below ground, no elevator. sorry"),
    ];

    let res_forms: serde_json::Value = client
        .post("https://httpbin.org/post")
        .body_form(&form_values)?
        .recv_json()
        .await?;

    for (name, value) in form_values.iter() {
        assert_eq!(res_forms["form"][name], *value);
    }

    let json_payload = json!({
        "book": "Rust 2018 Cookbook",
        "blog": "https://blog.x5ff.xyz",
    });

    let res_json: serde_json::Value = client
        .put("https://httpbin.org/anything")
        .body_json(&json_payload)?
        .recv_json()
        .await?;

    assert_eq!(res_json["json"], json_payload);

    let query_params = MyGetParams {
        a: 0x5ff,
        b: "https://blog.x5ff.xyz".into(),
    };
    let res_query: serde_json::Value = client
        .get("https://httpbin.org/get")
        .set_query(&query_params)?
        .recv_json()
        .await?;

    assert_eq!(res_query["args"]["a"], query_params.a.to_string());
    assert_eq!(res_query["args"]["b"], query_params.b);
    println!("> surf successful!");
    Ok(())
}

#[runtime::main]
async fn main() -> Result<(), Exception> {
    println!("Running some tests");
    test_reqwest()?;
    test_surf().await?;
    Ok(())
}

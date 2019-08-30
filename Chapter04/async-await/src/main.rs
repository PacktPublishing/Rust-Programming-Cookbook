use surf::Exception;
use surf::http::StatusCode;

async fn response_code(url: &str) -> Result<StatusCode, Exception> {
    let res = surf::get(url).await?;
    Ok(res.status())
}

#[runtime::main]
async fn main() -> Result<(), Exception> {
    let url = "https://www.rust-lang.org";
    let status = response_code(url).await?;
    println!("{} responded with HTTP {}", url, status);
    Ok(())
}
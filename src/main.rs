#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // This will POST a body of `foo=bar&baz=quux`
    let params = [("username", "Sergio"),("password","khkhgghdg6"),("age","127")];
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8000/login")
        .form(&params)
        .send()
        .await?;
        // .unwrap();
        println!("Status: {}", res.status());
        let body = res.text().await?;
        println!("Body:\n\n{}", body);
        Ok(())
}
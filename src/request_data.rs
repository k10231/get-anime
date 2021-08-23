use isahc::{prelude::*, Request};

pub fn notmain() -> Result<String, isahc::Error> {
    let mut response =
        Request::get("https://hurrxycxigvviayjhlxr.supabase.co/rest/v1/anime?select=link")
            .header("apikey", "anon-key")
            .header("Authorization", "Bearer token")
            .body(r#"{"never": "gonna","give": you up}"#,)?
            .send()?;
    let data = response.text().unwrap().replace("[", "").replace("]", "");
    Ok(data)
}

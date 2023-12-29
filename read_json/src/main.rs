use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn read_json_typed(raw_json: &str) -> Article{
    let parsed: Article = serde_json::from_str(raw_json).unwrap();// Parsing JSON to Article struct using serde_json crate
    return parsed;
}

fn main(){
    const JSON: &str = r#"
    {
    "article": "How to work with json in RUST",
    "author": "Priyanshu",
    "paragraph": [
        {
        "name": "This is the first Para"
        },
        {
        "name": "This is the second Para"
        },
        {
        "name": "This is the third Para"
        }
    ]
    }"#;

   let parsed: Article = read_json_typed(JSON); //Passing the raw JSON
   println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[0].name);
}
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

fn main(){
    // Creating Article struct
    let article: Article = Article{
        article: String::from("How to work with json in RUST"),
        author: String::from("Priyanshu"),
        paragraph: vec![
            Paragraph {
                name: String::from("This is the first para")
            },
            Paragraph {
                name: String::from("This is the second para")
            },
            Paragraph {
                name: String::from("This is the last para")
            }
        ]
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("The json is {}", json);
}
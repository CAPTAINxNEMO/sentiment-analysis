use std::{error::Error, io::{self, Write}};
use reqwest::Client;
use scraper::{Html, Selector};
use regex::Regex;
use csv::WriterBuilder;

fn preprocess_text(text: &str) -> String {
    let re = Regex::new(r"<br>|[\s]+").unwrap();
    re.replace_all(text, " ").to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print!("Enter the product reviews page link: ");
    io::stdout().flush().expect("Failed to flush.");
    let mut base_link = String::new();
    io::stdin().read_line(&mut base_link).expect("Failed to read input.");
    let base_link = base_link.trim();

    let client = Client::new();

    let mut page_number = 1;
    let mut link = format!("{}&page={}", base_link, page_number);

    let reviews_page_check = Selector::parse("div._23J90q.RcXBOT")?;
    let review_selector = Selector::parse("div.ZmyHeo > div > div")?;

    let mut reviews = vec![];

    if base_link.contains("flipkart.com") {
        if base_link.contains("/p/") {
            let response = client.get(base_link).header("Accept", "application/json").header("User-Agent", "Flipkart-Review-Collector/0.1").send().await?;
            let content = response.text().await?;
            let document = Html::parse_document(&content);

            if document.select(&review_selector).count() > 0 {
                for element in document.select(&review_selector) {
                    let original_text = element.text().collect::<Vec<_>>().join("").trim().replace("\"", "");
                    let preprocessed_text = preprocess_text(&original_text);
                    reviews.push(preprocessed_text);
                }

                let mut wtr = WriterBuilder::new().delimiter(b',').from_path("reviews.csv")?;
                wtr.write_record(["review", "score", "sentiment"])?;

                for review in &reviews {
                    wtr.write_record([review, "", ""])?;
                }

                wtr.flush()?;

                if document.select(&reviews_page_check).count() > 0 {
                    println!("\nMore reviews available on the reviews page.\n")
                }
            } else {
                println!("No reviews found for the product.")
            }
        } else if base_link.contains("/product-reviews/") {
            loop {
                let response = client.get(link).header("Accept", "application/json").header("User-Agent", "Flipkart-Review-Collector/0.1").send().await?;
                let content = response.text().await?;
                let document = Html::parse_document(&content);
        
                if document.select(&review_selector).count() == 0 {
                    println!("No reviews found on Page Number {}!", page_number);
                    break;
                } else {
                    println!("Page Number {} processed successfully!", page_number);
                }
        
                for element in document.select(&review_selector) {
                    let original_text = element.text().collect::<Vec<_>>().join("").trim().replace("\"", "");
                    let preprocessed_text = preprocess_text(&original_text);
                    reviews.push(preprocessed_text);
                }
        
                page_number += 1;
                link = format!("{}&page={}", base_link, page_number);
            }

            let mut wtr = WriterBuilder::new().delimiter(b',').from_path("reviews.csv")?;
            wtr.write_record(["review", "score", "sentiment"])?;

            for review in &reviews {
                wtr.write_record([review, "", ""])?;
            }

            wtr.flush()?;
        }
    } else {
        println!("Incorrect website!!\nPlease enter a Flipkart product review page link.")
    }

    Ok(())
}

use std::{error::Error, io::{self, Write}};
use reqwest::Client;
use scraper::{Html, Selector};
use vader_sentiment::SentimentIntensityAnalyzer;
use regex::Regex;
use csv::WriterBuilder;

struct ReviewData {
    review: String,
    score: f64,
    sentiment: String,
}

impl ReviewData {
    fn new(review: String) -> Self {
        let score = Self::calculate_score(&review);
        let sentiment = Self::determine_sentiment(score);

        Self {
            review,
            score,
            sentiment,
        }
    }

    fn calculate_score(review: &str) -> f64 {
        let analyzer = SentimentIntensityAnalyzer::new();
        let sentiment_scores = analyzer.polarity_scores(review);
        sentiment_scores["compound"]
    }

    fn determine_sentiment(score: f64) -> String {
        if score >= 0.05 {
            "Positive".to_string()
        } else if score <= -0.05 {
            "Negative".to_string()
        } else {
            "Neutral".to_string()
        }
    }
}

fn analytics(reviews: &[ReviewData]) {
    let total_reviews = reviews.len();
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut neutral_count = 0;

    for review in reviews {
        match review.sentiment.as_str() {
            "Positive" => positive_count += 1,
            "Negative" => negative_count += 1,
            "Neutral" => neutral_count += 1,
            _ => (),
        }
    }

    let average_score: f64 = reviews.iter().map(|r| r.score).sum::<f64>() / total_reviews as f64;
    let average_sentiment: String;
    if average_score >= 0.05 {
        average_sentiment = "Positive".to_string();
    } else if average_score <= -0.05 {
        average_sentiment = "Negative".to_string();
    } else {
        average_sentiment = "Neutral".to_string();
    }

    println!("Total Reviews              : {}", total_reviews);
    println!("Number of Positive Reviews : {}", positive_count);
    println!("Number of Negative Reviews : {}", negative_count);
    println!("Number of Neutral Reviews  : {}", neutral_count);
    println!("Average Sentiment Score    : {}", average_score);
    println!("Average Sentiment          : {}", average_sentiment);
}

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
            let response = client.get(base_link).header("Accept", "application/json").header("User-Agent", "Flipkart-Sentiment-Analysis/0.1").send().await?;
            let content = response.text().await?;
            let document = Html::parse_document(&content);

            if document.select(&review_selector).count() > 0 {
                for element in document.select(&review_selector) {
                    let original_text = element.text().collect::<Vec<_>>().join("").trim().replace("\"", "");
                    let preprocessed_text = preprocess_text(&original_text);
                    let review_data = ReviewData::new(preprocessed_text.to_string());
                    reviews.push(review_data);
                }

                let mut wtr = WriterBuilder::new().delimiter(b',').from_path("Reviews.csv")?;
                wtr.write_record(["Review", "Score", "Sentiment"])?;

                for review in &reviews {
                    println!("Review:\n{}", review.review);
                    println!("Score                      : {}", review.score);
                    println!("Sentiment                  : {}\n{}", review.sentiment, String::from("=").repeat(100));
                    wtr.write_record([&review.review, &review.score.to_string(), &review.sentiment])?;
                }

                wtr.flush()?;

                if document.select(&reviews_page_check).count() > 0 {
                    println!("\nMore reviews available on the reviews page.\n")
                }

                analytics(&reviews);
            } else {
                println!("No reviews found for the product.")
            }
        } else if base_link.contains("/product-reviews/") {
            loop {
                let response = client.get(link).header("Accept", "application/json").header("User-Agent", "Flipkart-Sentiment-Analysis/0.1").send().await?;
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
                    let review_data = ReviewData::new(preprocessed_text.to_string());
                    reviews.push(review_data);
                }
        
                page_number += 1;
                link = format!("{}&page={}", base_link, page_number);
            }

            let mut wtr = WriterBuilder::new().delimiter(b',').from_path("Reviews.csv")?;
            wtr.write_record(["Review", "Score", "Sentiment"])?;

            for review in &reviews {
                println!("Review:\n{}", review.review);
                println!("Score                      : {}", review.score);
                println!("Sentiment                  : {}\n{}", review.sentiment, String::from("=").repeat(100));
                wtr.write_record([&review.review, &review.score.to_string(), &review.sentiment])?;
            }

            wtr.flush()?;

            analytics(&reviews);
        }
    } else {
        println!("Incorrect website!!\nPlease enter a Flipkart product review page link.")
    }

    Ok(())
}

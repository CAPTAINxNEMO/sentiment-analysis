# Flipkart Sentiment Analysis

A tool to scrape product reviews from Flipkart and analyze their sentiment using VADER (Valence Aware Dictionary and sEntiment Reasoner).

## Project Overview

This project consists of two main components:
1. A Rust scraper to collect product reviews from Flipkart
2. A Python sentiment analyzer to process and categorize the collected reviews

## Features

- Scrape product reviews from Flipkart product pages or review pages
- Handle multiple pages of reviews automatically
- Preprocess text to clean up formatting issues
- Analyze sentiment using VADER sentiment analysis
- Categorize reviews as Positive, Negative, or Neutral
- Calculate sentiment scores for individual reviews and overall product sentiment
- Save results to CSV for further analysis

## Requirements

### Rust Dependencies
- csv = "1.3.0"
- regex = "1.10.6"
- reqwest = "0.12.5"
- scraper = "0.20.0"
- tokio = "1.39.2" (with full features)

### Python Dependencies
- pandas
- vaderSentiment

## Installation

1. Clone the repository:
```bash
git clone https://github.com/CAPTAINxNEMO/sentiment-analysis
cd sentiment-analysis
```

2. Install Rust dependencies:
```bash
cargo build
```

3. Install Python dependencies:
```bash
pip install pandas vaderSentiment
```

## Usage

### Using the Batch Script (Windows)

Run the included batch script to execute both components in sequence:
```bash
.\main.bat
```

### Manual Execution

1. Run the Rust scraper:
```bash
cargo run
```
When prompted, enter a Flipkart product URL. Two URL formats are supported:
- Product page: `https://www.flipkart.com/product-name/p/item-id`
- Review page: `https://www.flipkart.com/product-name/product-reviews/item-id`

2. Run the Python sentiment analyzer:
```bash
python main.py
```

## Output

The program will:
1. Create a CSV file (Reviews.csv) with the scraped reviews
2. Analyze each review and assign a sentiment score and label
3. Print detailed sentiment analysis for each review
4. Provide summary statistics of all reviews

## Example Output

```
Review: Great product, works as expected!
Overall Sentiment Dictionary: {'neg': 0.0, 'neu': 0.508, 'pos': 0.492, 'compound': 0.6249}
Positive Score: 49.2000 %
Negative Score: 0.0000 %
Neutral Score: 50.8000 %
Overall Rating: Positive

Number of Positive Reviews : 42
Number of Negative Reviews : 8
Number of Neutral Reviews  : 5
Total Reviews              : 55
Overall Sentiment Score: 0.4217
Overall Sentiment: Positive
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Shashank Bhave
from pandas import read_csv
from vaderSentiment.vaderSentiment import SentimentIntensityAnalyzer

def sentiment_scores(review):
    global positive_count
    global negative_count
    global neutral_count

    sid_obj = SentimentIntensityAnalyzer()
    sentiment_dict = sid_obj.polarity_scores(review)
    print(f'Review: {review}')
    print(f'Overall Sentiment Dictionary: {sentiment_dict}')
    print(f'Positive Score: {sentiment_dict["pos"] * 100:.4f} %')
    print(f'Negative Score: {sentiment_dict["neg"] * 100:.4f} %')
    print(f'Neutral Score: {sentiment_dict["neu"] * 100:.4f} %')
    print(f'Overall Rating:', end = ' ')
    if sentiment_dict['compound'] >= 0.055:
        print('Positive\n')
        positive_count += 1
    elif sentiment_dict['compound'] <= -0.055:
        print('Negative\n')
        negative_count += 1
    else:
        print('Neutral\n')
        neutral_count += 1
    
    return sentiment_dict['compound'], get_sentiment_label(sentiment_dict['compound'])

def get_sentiment_label(compound_score):
    if compound_score >= 0.055:
        return 'Positive'
    elif compound_score <= -0.055:
        return 'Negative'
    else:
        return 'Neutral'

def avg_sentiment(reviews_dataset):
    sid = SentimentIntensityAnalyzer()
    positive_scores = []
    negative_scores = []
    neutral_scores = []
    compound_scores = []
    
    for review in reviews_dataset:
        sentiment_scores = sid.polarity_scores(review)
        positive_scores.append(sentiment_scores['pos'])
        negative_scores.append(sentiment_scores['neg'])
        neutral_scores.append(sentiment_scores['neu'])
        compound_scores.append(sentiment_scores['compound'])

    compound_score_avg = sum(compound_scores) / len(compound_scores)

    print(f'Overall Sentiment Score: {compound_score_avg:.4f}')
    print('Overall Sentiment:', end = ' ')
    if compound_score_avg >= 0.055:
        print('Positive')
    elif compound_score_avg <= -0.055:
        print('Negative')
    else:
        print('Neutral')

if __name__ == '__main__':
    positive_count = 0
    negative_count = 0
    neutral_count = 0

    data = read_csv('reviews.csv')

    for index, row in data.iterrows():
        review = row['review']
        score, sentiment_label = sentiment_scores(review)
        data.at[index, 'score'] = score
        data.at[index, 'sentiment'] = sentiment_label
    data.to_csv('reviews.csv', index = False)

    total_reviews = positive_count + negative_count + neutral_count

    print(f'Number of Positive Reviews : {positive_count}')
    print(f'Number of Negative Reviews : {negative_count}')
    print(f'Number of Neutral Reviews  : {neutral_count}')
    print(f'Total Reviews              : {total_reviews}')

    reviews = data['review']
    avg_sentiment(reviews)

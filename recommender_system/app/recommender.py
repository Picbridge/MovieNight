from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics.pairwise import cosine_similarity
import logging
import json

desired_keys = [
    'id',
    'title',
    'description',
    'imdb_rating',
    'stars',
    'image_url',
    'released_year',
    'runtime',
    'metadata',
    'genres',
    'director',
]

# Fetch all movies
def fetch_movies(container):
    query = "SELECT * FROM c WHERE c.type = 'movie'"
    movies = list(container.query_items(query, enable_cross_partition_query=True))
    return movies

def recommend_movies(container, input_data, top_n=5):
    # Fetch movies
    #logging.info(f"Received recommendation request: {input_data}")
    
    # Deserialize the JSON string into a Python dictionary
    try:
        data = json.loads(input_data)
    except json.JSONDecodeError as e:
        logging.error(f"Invalid JSON data: {e}")
        return []
    
    # Extract user-selected movies
    selected_movies = data.get('movies', [])
    
    # Extract knowledge-based criteria
    year_range = data.get('year_range', [])
    if len(year_range) == 2:
        year_start, year_end = year_range
    else:
        year_start, year_end = None, None
    
    runtime_range = data.get('runtime_range', [])
    if len(runtime_range) == 2:
        runtime_start, runtime_end = runtime_range
    else:
        runtime_start, runtime_end = None, None
    
    rating = data.get('rating', None)
    
    # Load all movies from the database
    all_movies = fetch_movies(container)
    
    # Apply knowledge-based filtering first to reduce the number of candidate movies
    candidate_movies = []
    for movie in all_movies:
        try:
            # Skip if the movie is one of the selected movies
            if movie['id'] in [m['id'] for m in selected_movies]:
                continue
            
            released_year = int(movie.get('released_year', 0))
            runtime = int(movie.get('runtime', '0').split()[0])
            imdb_rating = float(movie.get('imdb_rating', 0.0))
            
            # Ignore movies that do not meet the criteria
            if year_start and year_end and not (year_start <= released_year <= year_end):
                continue
            if runtime_start and runtime_end and not (runtime_start <= runtime <= runtime_end):
                continue
            if rating and imdb_rating < rating:
                continue
            
            candidate_movies.append(movie)
        except (ValueError, TypeError) as e:
            logging.warning(f"Skipping movie due to error: {e}")
            continue
    
    if not candidate_movies:
        logging.info("No candidate movies found after applying filters.")
        return []
    
    # Combine selected movies and candidate movies for vectorization
    combined_movies = selected_movies + candidate_movies
    
    # Create a collection of movie metadata
    collection = [movie.get('metadata', '') for movie in combined_movies]
    
    # Go through calculations to find the most similar movies
    vectorizer = TfidfVectorizer(stop_words='english')
    tfidf_matrix = vectorizer.fit_transform(collection)
    
    num_selected = len(selected_movies)
    similarity_scores = cosine_similarity(tfidf_matrix[:num_selected], tfidf_matrix[num_selected:])
    
    # Average the similarity scores across all selected movies
    avg_similarity = similarity_scores.mean(axis=0)
    
    top_indices = avg_similarity.argsort()[::-1][:top_n]
    
    recommended_movies = [candidate_movies[i] for i in top_indices]
    
    filtered_recommended_movies = [
        {key: movie[key] for key in desired_keys if key in movie}
        for movie in recommended_movies
    ]
    movies_json = json.dumps(filtered_recommended_movies)
    #logging.info(f"Recommended movies: {movies_json}")

    # Return the recommended movies
    return movies_json
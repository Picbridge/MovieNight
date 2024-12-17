from flask import Blueprint, request, jsonify, current_app
from . import llm
from . import recommender
import logging
from flask_cors import CORS
from app.config import Config

main = Blueprint('main', __name__)
CORS(main)

logging.basicConfig(
    level=logging.INFO,  
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler() 
    ]
)

@main.route('/reasoning', methods=['POST'])
def reasoning():
    input = request.data.decode('utf-8').strip()
    reason = llm.generate_reasoning(input)

    return jsonify({'reason': reason})
    

@main.route('/generate', methods=['POST'])
def generate():
    movie_name = request.data.decode('utf-8').strip()
    logging.info(f"Movie name received: {movie_name}")
    prompt = (
    'List websites as many as you can where I can stream {movie_name} legally. Your answer should only be the names of websites SEPARATED BY COMMA, WITHOUT repeating the question or adding explanations. Example format: '
    '"Netflix", "Amazon Prime", ... \n\nAnswer: '
    ).format(movie_name=movie_name)
    try:
        # Generate response from Llama
        response = llm.generate_response(prompt)
        
        logging.info(f"Response generated: {response}")
        
        return jsonify({'response': response})
    except Exception as e:
        # Handle exceptions and return an error response
        return jsonify({'error': f'An error occurred: {str(e)}'}), 500

@main.route('/fetch_movies')
def fetch_movies():
    container = Config.COSMOS_DB_CONTAINER
    movies = recommender.fetch_movies(container)
    return jsonify(movies)

@main.route('/recommend', methods=['POST'])
def recommend():
    container = Config.COSMOS_DB_CONTAINER
    input = request.data.decode('utf-8').strip()

    try:
        recommendations = recommender.recommend_movies(container=container, input_data=input)
        return recommendations
    except Exception as e:
        logging.error(f"Error generating recommendations: {e}", exc_info=True)
        return jsonify({'error': 'Failed to generate recommendations'}), 500

import subprocess
import logging
import google.generativeai as genai
from app.config import Config
import json

genai.configure(api_key=Config.GEMINI_API_KEY)

model = genai.GenerativeModel(model_name="gemini-1.5-flash")

reasoning_keys = [
    'selected_movie',
    'recommended_movie'
]

def generate_response(prompt):
    try:
        logging.info(f"Generating response for prompt: {prompt}")
        response = model.generate_content(prompt)

        return response.text
    except subprocess.CalledProcessError as e:
        print("Error executing LLM:", e.stderr)
        return "Error executing LLM model"
    
def generate_reasoning(input):
    try:
        data = json.loads(input)
    except json.JSONDecodeError as e:
        logging.error(f"Invalid JSON data: {e}")
        return "Invalid input data."

    # Extract the correct keys from the input
    selected_movies = data.get('selected_movie', [])
    recommended_movies = data.get('recommended_movie', [])

    # Handle empty inputs gracefully
    if not selected_movies or not recommended_movies:
        logging.warning("Selected or recommended movies are empty.")
        return "No movies available for reasoning."

    # Generate strings for the prompt
    selected_movies_str = ', '.join(selected_movies)
    recommended_movies_str = ', '.join(recommended_movies)

    prompt = f"""Provide reasoning for recommending {recommended_movies_str} based on {selected_movies_str}.
    DO NOT USE MARKDOWN OR HTML IN YOUR RESPONSE. PLAIN TEXT ONLY.
    You need to let user know which movies were selected and why the recommended movies are relevant to the selected movies.
    Note that the recommender is using a content-based filtering approach to recommend movies based on the selected movies.
    Do not mention anything technical or related to the recommender system itself. Just provide a simple, human-readable explanation."""

    logging.info(f"Generating response for prompt: {prompt}")

    # Assuming `model.generate_content` generates reasoning
    try:
        response = model.generate_content(prompt)
        if hasattr(response, 'text'):
            return response.text
        else:
            logging.error("Response does not have a 'text' attribute")
            return "Failed to generate reasoning."
    except Exception as e:
        logging.error(f"Failed to generate reasoning: {e}")
        return "Error generating reasoning."
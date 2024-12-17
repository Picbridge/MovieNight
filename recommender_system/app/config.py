import os
from dotenv import load_dotenv
from azure.cosmos import CosmosClient

load_dotenv()

class Config:
    COSMOS_DB_URL = os.getenv("COSMOS_DB_URL")
    COSMOS_DB_KEY = os.getenv("COSMOS_DB_KEY")
    DATABASE_NAME = os.getenv("DATABASE_NAME")
    CONTAINER_NAME = os.getenv("CONTAINER_NAME")

    GEMINI_API_KEY = os.getenv("GEMINI_API_KEY")
    
    client = CosmosClient(COSMOS_DB_URL, COSMOS_DB_KEY)
    database = client.get_database_client(DATABASE_NAME)
    COSMOS_DB_CONTAINER = database.get_container_client(CONTAINER_NAME)

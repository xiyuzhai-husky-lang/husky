"""
OpenAI API Wrapper with Caching
------------------------------
This module provides a caching layer for OpenAI API calls with the following behavior:
- Caches API responses based on model and message content
- Requires ENABLE_API_CALLING=1 in .env to make actual API calls
- Uses local JSON file for caching
- Cache key is based on model name and full message content
"""

from dotenv import load_dotenv
import os
from openai import OpenAI
import json
from pathlib import Path

load_dotenv()  # Load environment variables from .env file

# Global flags
API_ENABLED = os.getenv('ENABLE_API_CALLING') == '1'

# Add at the top of the file, after other imports
class Colors:
    RED = '\033[91m'
    GREEN = '\033[92m'
    RESET = '\033[0m'

def initialize_client():
    """Initialize OpenAI client if API calling is enabled."""
    if API_ENABLED:
        return OpenAI(api_key=os.getenv('OPENAI_API_KEY'))
    return None

# Initialize client based on API_ENABLED flag
client = initialize_client()

def call_openai_api(model: str, messages: list) -> dict:
    """Make API call to OpenAI if enabled."""
    if not API_ENABLED:
        raise ValueError(f"{Colors.RED}API calling is disabled. Set ENABLE_API_CALLING=1 to enable.{Colors.RESET}")
    
    if client is None:
        raise ValueError(f"{Colors.RED}OpenAI client is not initialized{Colors.RESET}")
    
    completion = client.chat.completions.create(
        model=model,
        messages=messages
    )
    return {
        "content": completion.choices[0].message.content,
        "role": completion.choices[0].message.role
    }

def get_cached_completion(cache_path: str, model: str, messages: list) -> dict:
    """
    Get completion from cache or API, then cache the result.
    """
        
    # Validate cache directory exists
    cache_file = Path(cache_path)
    if not cache_file.parent.exists():
        raise FileNotFoundError(f"{Colors.RED}Cache directory {cache_file.parent} does not exist{Colors.RESET}")
    
    # Create a cache key from the model and messages
    cache_key = {
        "model": model,
        "messages": messages
    }
    
    # Try to load from cache
    if cache_file.exists():
        with open(cache_file, 'r') as f:
            cache = json.load(f)
            cache_str = json.dumps(cache_key, sort_keys=True)
            if cache_str in cache:
                return cache[cache_str]
    else:
        cache = {}

    # If not in cache, call API using the existing function
    completion_dict = call_openai_api(model, messages)
    
    # Update cache
    cache[json.dumps(cache_key, sort_keys=True)] = completion_dict
    with open(cache_file, 'w') as f:
        json.dump(cache, f, indent=2)
    
    return completion_dict

result = get_cached_completion(
    cache_path="./cache.json",
    model="gpt-4",  # Fixed typo in model name
    messages=[
        {"role": "system", "content": "You are a helpful assistant."},
        {"role": "user", "content": "Write a haiku about recursion in programming."}
    ]
)

if __name__ == "__main__":
    print("Response: ")
    indent = "  "
    indented_content = "\n".join(f"{indent}{line}" for line in result['content'].split('\n'))
    print(f"{Colors.GREEN}{indented_content}{Colors.RESET}")

from dotenv import load_dotenv
import os
from openai import OpenAI

load_dotenv()  # Load environment variables from .env file

client = OpenAI(
    api_key=os.getenv('OPENAI_API_KEY')
)

completion = client.chat.completions.create(
    model="gpt-4o",
    messages=[
        {"role": "system", "content": "You are a helpful assistant."},
        {
            "role": "user",
            "content": "Write a haiku about recursion in programming."
        }
    ]
)

print(completion.choices[0].message)
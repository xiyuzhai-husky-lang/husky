from openai import OpenAI
import hashlib
from diskcache import Cache

cache = Cache(".chat_completion_cache")

class ChatCompletionAPI:
    def __init__(self, type, model=None):
        if type == "local":
            self.client = OpenAI(base_url="http://172.28.6.60:8000/v1") # DO NOT CHANGE THIS LINE
            self.model = "/local1/models/models--meta-llama--Llama-3.1-70B-Instruct/snapshots/945c8663693130f8be2ee66210e062158b2a9693"  # DO NOT CHANGE THIS LINE
        elif type == "openai":
            self.client = OpenAI()
            self.model = model
        else:
            raise ValueError("Invalid type")

    def _generate_cache_key(self, model, messages):
        messages_str = "".join([msg["role"] + msg["content"] for msg in messages])
        key_base = f"{model}:{messages_str}"
        cache_key = hashlib.md5(key_base.encode()).hexdigest()
        return cache_key

    def _fetch_completion(self, model, messages, use_cache):
        if use_cache:
            cache_key = self._generate_cache_key(model, messages)
            cached_result = cache.get(cache_key)
            if cached_result:
                return cached_result
        
        completion = self.client.chat.completions.create(
            model=model,
            messages=messages
        )
        result = {
            "role": completion.choices[0].message.role,
            "content": completion.choices[0].message.content,
        }
        cache.set(cache_key, result)
        return result

    def chat_completion(self, messages, use_cache=True):
        return self._fetch_completion(self.model, messages, use_cache)

if __name__ == "__main__":
    api = ChatCompletionAPI("local")

    messages = [
        {"role": "system", "content": "You are a helpful assistant."},
        {"role": "user", "content": "What is the capital of France?"},
    ]
    for _ in range(3):
        completion = api.chat_completion(messages)
        print(completion["content"])

        messages.append(completion)
        messages.append({
            "role": "user",
            "content": "Tell me more fun facts about this city.",
        })

from prompts import SYSTEM_MESSAGE, prompt
from api import ChatCompletionAPI

api = ChatCompletionAPI("local")

messages = [
    {"role": "system", "content": SYSTEM_MESSAGE},
    {"role": "user", "content": prompt(
        problem="For any $x \in \mathbb{R}$, $x \ge 0$, prove that $x + 1 \ge 2 \sqrt x$.",
        latex="Let $x\in\mathbb{R}, x \ge 0$. Then ${\sqrt x}^2 + 1 - 2 \sqrt x = {(\sqrt x - 1)}^2$. Then ${(\sqrt x - 1)}^2 \ge 0$. Then $x + 1 \ge 2 \sqrt x$.",
        lean="",
    )},
]

completion = api.chat_completion(messages)
print(completion["content"])

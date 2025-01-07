def parse_testcase(testcase):
    INDICATORS = ["problem:", "latex proof:", "lean proof:"]

    testcase_lower = testcase.lower()
    positions = [testcase_lower.find(indicator) for indicator in INDICATORS] + [len(testcase)]
    parts = []
    for i in range(len(INDICATORS)):
        if positions[i] == -1:
            raise ValueError(f"Indicator '{INDICATORS[i]}' not found in testcase")
        start = positions[i] + len(INDICATORS[i])
        end = positions[i + 1]
        parts.append(testcase[start:end].strip())
    
    # special handle lean
    lean = parts[2].split("\n")
    parts[2] = ("\n".join(lean[1: -1])).strip()

    return parts

def find_all(s, sub):
    start = 0
    while True:
        start = s.find(sub, start)
        if start == -1:
            break
        yield start
        start += len(sub)


def parse_response(response):
    positions = list(find_all(response, "```"))
    if len(positions) % 2 != 0 or len(positions) == 0:
        # Return a Lean error statement instead of raising an exception
        # print(response)
        # raise ValueError("No paired '```' found in response")
        print(response)
        print("No paired '```' found in response")
        return 'theorem error_no_code_block : false := by\n  sorry\n  -- ERROR: No code block found in response:\n  -- ' + response.replace('\n', '\n  -- ')
    
    if len(positions) > 2:
        print("More than one pair of '```' found in response, default to the last pair")
    
    code_block = response[positions[-2]: positions[-1] + 3].split("\n")

    return "\n".join(code_block[1: -1]).strip()

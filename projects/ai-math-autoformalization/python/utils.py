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
    if len(positions) != 2:
        print(response)
        raise ValueError("No paired '```' found in response")
    
    code_block = response[positions[0]: positions[1] + 3].split("\n")

    main_func = \
'''
def main : IO Unit :=
  IO.println "Success!"
'''

    return "\n".join(code_block[1: -1]).strip() + main_func
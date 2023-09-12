# 1 + (2 * 3) + (4 * (5 + 6))
# 1 + 2 * 3 + 4 * 5 + 6
# 1 + (2 * 3)
# (3 * 4) + (5 * 6)
# "5 + (8 * 3 + 9 + 3 * 4 * 3)
# 3 4 * 5 6 * +
# 123*+     

def tokenize(expression):
    tokens = []
    stack = []
    for char in expression:
        if char.isnumeric():
            tokens.append(char)
        else:
            if char == "+" :
                stack.append(char)
            if char == '*' :
                while stack and stack[-1] != '(':
                    tokens.append(stack.pop())
                stack.append(char)
            elif char == '(':
                stack.append(char)
            elif char == ')':
                operator = stack.pop()
                while operator != '(':
                    tokens.append(operator)
                    operator = stack.pop()
    while stack:
        operator = stack.pop()
        tokens.append(operator)
    return tokens

def calc(tokens):
    stack = []
    print(tokens)
    for token in tokens:
        if token.isnumeric():
            stack.append(token)
        else:
            x = stack.pop()
            y = stack.pop()
            cal = eval(f"{x}{token}{y}")
            stack.append(cal)
    return stack[-1] if stack else 0

def main():
    with open("./input/q18.txt") as f:
        lines = f.readlines()
    # expression = "(3 * 4) + (5 * 6)".replace(" ", "")
    # # expression = "1 + (2 * 3)".replace(" ", "")
    # expression = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".replace(" ", "")
    # # expression = "5 + (8 * 3 + 9 + 3 * 4 * 3)".replace(" ", "")
    # expression = "1 + 2 * 3 + 4 * 5 + 6".replace(" ", "")
    # print(expression)
    result = []
    for line in lines:
        expression = line.replace(" ", "")
        print(expression)
        tokens = tokenize(expression)
        cal = calc(tokens)
        result.append(cal)
    print(sum(result))
if __name__ == "__main__":
    main()
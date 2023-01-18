import sys
import random


def generate_pattern(length):
    for i in range(1, length + 1):
        num1 = random.uniform(0.1, 100)
        num2 = random.uniform(0.1, 100)
        result = random.choice(["true", "false"])
        print(f"{i}, {num1:.1f}, {num2:.1f}, {result}")


def main():
    length = int(sys.argv[1])
    generate_pattern(length)


if __name__ == "__main__":
    main()

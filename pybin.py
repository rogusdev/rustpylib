import rustpylib

def main():
    name = input("Enter your name: ")
    greeting = rustpylib.greet(name)
    print(greeting)
    a = input("Enter a first integer: ")
    b = input("Enter another integer: ")
    sum = rustpylib.sum_as_string(int(a), int(b))
    print(sum)

if __name__ == "__main__":
    main()

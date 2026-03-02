import time
import sys
sys.set_int_max_str_digits(0)  # 0 means unlimited

def convert(temperature: float) -> float:
    return temperature * 1.8 + 32.0

def fibonacci(n: int) -> int:
    if n == 0:
        return 0
    elif n == 1:
        return 1
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b

def main():
    temp = float(input("Enter temperature in Celsius: "))
    temp_f = convert(temp)
    print(f"Temperature in Fahrenheit is {temp_f}")

    n = int(input("Enter n to get nth Fibonacci number: "))
    start = time.time()
    fib_n = fibonacci(n)
    end = time.time()
    print(f"The {n}th Fibonacci number is {fib_n}")
    print(f"Python Fibonacci calculation took {end - start:.8f} seconds")

if __name__ == "__main__":
    main()

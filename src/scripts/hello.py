#!/usr/bin/env python3
import sys
import requests

def main():
    print(f"Hello from Python! Args: {sys.argv[1:]}")
    print(f"Requests version: {requests.__version__}")
    
    # Echo back all arguments
    for i, arg in enumerate(sys.argv[1:]):
        print(f"Arg {i}: {arg}")

if __name__ == "__main__":
    main()
import requests
import time

def test_server():
    url = "http://localhost:8000"
    max_retries = 5
    retry_delay = 2

    for attempt in range(max_retries):
        try:
            response = requests.get(url)
            if response.status_code == 200:
                print("Server is running!")
                print("Response:", response.json())
                return True
        except requests.exceptions.ConnectionError:
            print(f"Attempt {attempt + 1}/{max_retries}: Server not ready, retrying in {retry_delay} seconds...")
            time.sleep(retry_delay)
    
    print("Failed to connect to server after multiple attempts")
    return False

if __name__ == "__main__":
    test_server() 
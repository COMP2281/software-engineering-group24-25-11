import csv
import json

FILE_PATH = "WebDev.csv"
OUT_FILE_PATH = "./web_dev.json"
if __name__ == "__main__":
    try:
        with open(FILE_PATH, "r", newline="", encoding="utf-8") as file:
            reader = csv.DictReader(file)
            result = []
            for row in reader:
                result.append(
                    {
                        "type": "single",
                        "question": row["Question"],
                        "choices": [
                            row["Choice1"],
                            row["Choice2"],
                            row["Choice3"],
                            row["Choice4"],
                        ],
                        "answer": row["Answer"],
                        "difficulty": 0,
                    }
                )

            with open(OUT_FILE_PATH, "w", encoding="utf-8") as out_file:
                out_file.write(json.dumps(result))
    except Exception as e:
        print("Error", f"Failed to load file: {e}")

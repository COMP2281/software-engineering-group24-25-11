import pandas as pd
from dataclasses import dataclass
from typing import Optional
import numpy as np

@dataclass
class Question:
    question: str
    choice1: str
    choice2: str
    choice3: str
    choice4: str
    answer: str
    difficulty: Optional[float] = 0.5

class User:
    def __init__(self, name: str, score: float, difficulty: float):
        self.name = name
        self.score = score
        self.difficulty = difficulty

def main():

    # set filepath for later use when multiple question banks are implemented
    def setFilepath(filepath: str):
        file_path = "quiz_logic/question_bank/" + filepath + ".csv"

    file_path = "quiz_logic/question_bank/ai_modules.csv"
    df = pd.read_csv(file_path)
    selectedQuestions = []
    
    df.columns = [col.strip() for col in df.columns]  

    # Fill NaN values with 0.5
    df['Difficulty'] = pd.to_numeric(df.get('Difficulty', 0.5), errors='coerce').fillna(0.5)
    
    # Parse questions into objects
    questions = [
        Question(
            question=row['Question'],
            choice1=row['Choice1'],
            choice2=row['Choice2'],
            choice3=row['Choice3'],
            choice4=row['Choice4'],
            answer=row['Answer'],
            difficulty=row['Difficulty']
        )
        for _, row in df.iterrows()
    ]


    # "adaptive difficulty" algorithm, adjust user score, as well as difficulty accordingly when answer is correct or false
    def evaulateAnswer(user: User, question: Question, answer: str) -> bool:
        if question.answer == answer:
            user.score += question.difficulty * 100
            user.difficulty += question.difficulty * 0.1
            if user.difficulty > 1:
                user.difficulty = 1
            question.difficulty -= user.difficulty * 0.1
            if question.difficulty < 0:
                question.difficulty = 0
        else:
            question.difficulty += user.difficulty * 0.1
            if question.difficulty > 1:
                question.difficulty = 1
            user.difficulty -= question.difficulty * 0.1
            if user.difficulty < 0:
                user.difficulty = 0
        return 1
    
    # get question based on user difficulty
    def getQuestion(user: User, questions) -> Question:
        userDifficulty = user.difficulty
        randSeed = userDifficulty + np.random.uniform(-0.15, 0.15)
        if randSeed < 0:
            randSeed = 0
        elif randSeed > 1:
            randSeed = 1

        # return question with lowest difference to randSeed
        selectedQ = min(questions, key=lambda q: abs(q.difficulty - randSeed))

        # remove selected question from list of questions and add it to selectedQuestions
        questions.remove(selectedQ)
        selectedQuestions.append(selectedQ)
        return selectedQ
    
    
    

    

if __name__ == "__main__":
    main()

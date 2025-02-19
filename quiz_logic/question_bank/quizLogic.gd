extends Node

class Question:
	var question: String
	var choice1: String
	var choice2: String
	var choice3: String
	var choice4: String
	var answer: String
	var difficulty: float

	func _init(q: String, c1: String, c2: String, c3: String, c4: String, ans: String, diff: float = 0.5):
		question = q
		choice1 = c1
		choice2 = c2
		choice3 = c3
		choice4 = c4
		answer = ans
		difficulty = diff

class User:
	var name: String
	var score: float
	var difficulty: float

	func _init(n: String, s: float, d: float):
		name = n
		score = s
		difficulty = d

	func set_difficulty(d: String):
		if d == "easy":
			difficulty = 0.3
		elif d == "medium":
			difficulty = 0.5
		elif d == "hard":
			difficulty = 0.7

var questions = []
var selected_questions = []

func filePathChoose(bank: string):
	var filepath = "res://quiz_logic/question_bank/" + bank + ".json"
	return filepathx	


func _ready():
	load_questions("res://quiz_logic/question_bank/ai_modules.json")




func load_questions(file_path: String):
	var file = FileAccess.open(file_path, FileAccess.READ)
	if not file:
		print("Error loading file")
		return

	var content = file.get_as_text()
	var data = JSON.parse_string(content)
	if not data:
		print("Error parsing JSON")
		return

	for entry in data:
		var diff = float(entry.get("difficulty", 0.5))
		questions.append(Question.new(entry["question"], entry["choice1"], entry["choice2"], entry["choice3"], entry["choice4"], entry["answer"], diff))

	file.close()

func evaluate_answer(user: User, question: Question, answer: String):
	if question.answer == answer:
		user.score += question.difficulty * 100
		user.difficulty = min(1.0, user.difficulty + question.difficulty * 0.1)
		question.difficulty = max(0.0, question.difficulty - user.difficulty * 0.1)
	else:
		question.difficulty = min(1.0, question.difficulty + user.difficulty * 0.1)
		user.difficulty = max(0.0, user.difficulty - question.difficulty * 0.1)
	return true

func get_question(user: User):
	var user_difficulty = user.difficulty
	var rand_seed = user_difficulty + randf_range(-0.15, 0.15)
	rand_seed = clamp(rand_seed, 0, 1)
	var selected_q = questions[0]
	var min_diff = abs(selected_q.difficulty - rand_seed)

	for q in questions:
		var diff = abs(q.difficulty - rand_seed)
		if diff < min_diff:
			min_diff = diff
			selected_q = q

	questions.erase(selected_q)
	selected_questions.append(selected_q)
	return selected_q

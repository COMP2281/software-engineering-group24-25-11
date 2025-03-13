import json
import tkinter as tk
from tkinter import filedialog, messagebox, ttk

class QuestionBankTool:
    def __init__(self, root):
        self.root = root
        self.root.title("Question Bank Tool")

        # UI Components
        self.file_label = tk.Label(root, text="No file selected")
        self.file_label.pack()

        self.create_button = tk.Button(root, text="Create New JSON", command=self.create_json)
        self.create_button.pack()

        self.load_button = tk.Button(root, text="Load JSON", command=self.load_json)
        self.load_button.pack()

        self.question_listbox = tk.Listbox(root, width=100, height=15)
        self.question_listbox.pack()

        self.add_button = tk.Button(root, text="Add Question", command=self.add_question)
        self.add_button.pack()

        self.edit_button = tk.Button(root, text="Edit Selected Question", command=self.edit_question)
        self.edit_button.pack()

        self.delete_button = tk.Button(root, text="Delete Selected Question", command=self.delete_question)
        self.delete_button.pack()

        self.save_button = tk.Button(root, text="Save JSON", command=self.save_json)
        self.save_button.pack()

        self.questions = []
        self.file_path = None

    def create_json(self):
        self.file_path = filedialog.asksaveasfilename(defaultextension=".json", filetypes=[("JSON files", "*.json")])
        if not self.file_path:
            return

        try:
            with open(self.file_path, "w", encoding="utf-8") as file:
                json.dump([], file)
            self.file_label.config(text=f"Created: {self.file_path}")
            self.questions.clear()
            self.question_listbox.delete(0, tk.END)
            messagebox.showinfo("Success", "New JSON file created successfully.")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to create file: {e}")

    def load_json(self):
        self.file_path = filedialog.askopenfilename(filetypes=[("JSON files", "*.json")])
        if not self.file_path:
            return

        self.questions.clear()
        self.question_listbox.delete(0, tk.END)

        try:
            with open(self.file_path, "r", encoding="utf-8") as file:
                self.questions = json.load(file)
                for q in self.questions:
                    self.question_listbox.insert(tk.END, q["question"])
            self.file_label.config(text=f"Loaded: {self.file_path}")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to load file: {e}")

    def add_question(self):
        self.edit_question(new=True)

    def edit_question(self, new=False):
        def update_answer_ui():
            # Show/hide answer widgets based on question type
            if question_type.get() == "single":
                single_answer_frame.grid()
                multiple_answer_frame.grid_remove()
            else:
                single_answer_frame.grid_remove()
                multiple_answer_frame.grid()
                # Clear previous selections
                multiple_answer_listbox.selection_clear(0, tk.END)

        def save():
            # Validate question text
            question_text = question_entry.get("1.0", tk.END).strip()
            if not question_text:
                messagebox.showerror("Error", "Question cannot be empty.")
                return

            # Validate choices
            choices = [choice_entry.get("1.0", tk.END).strip() for choice_entry in choice_entries]
            if not all(choices):
                messagebox.showerror("Error", "All choices must be filled.")
                return

            # Validate difficulty
            try:
                difficulty = float(difficulty_entry.get())
                if not (0.0 <= difficulty <= 1.0):
                    raise ValueError()
            except:
                messagebox.showerror("Error", "Difficulty must be a float between 0.0 and 1.0")
                return

            # Get answer(s)
            if question_type.get() == "single":
                answer = single_answer_var.get()
                if not (0 <= answer < len(choices)):
                    messagebox.showerror("Error", "Invalid answer index")
                    return
            else:
                answer = list(multiple_answer_listbox.curselection())
                if not answer:
                    messagebox.showerror("Error", "Select at least one answer")
                    return

            # Build question object
            question = {
                "type": question_type.get(),
                "question": question_text,
                "choices": choices,
                "answer": answer if question_type.get() == "multiple" else answer[0],
                "difficulty": difficulty
            }

            # Update list
            if new:
                self.questions.append(question)
                self.question_listbox.insert(tk.END, question["question"])
            else:
                index = self.question_listbox.curselection()[0]
                self.questions[index] = question
                self.question_listbox.delete(index)
                self.question_listbox.insert(index, question["question"])
            
            editor.destroy()

        # --- UI Setup ---
        if not new and not self.question_listbox.curselection():
            messagebox.showwarning("Warning", "No question selected.")
            return

        editor = tk.Toplevel(self.root)
        editor.title("Question Editor")
        
        # Question Type
        tk.Label(editor, text="Question Type:").grid(row=0, column=0, sticky=tk.W)
        question_type = ttk.Combobox(editor, values=["single", "multiple"], state="readonly")
        question_type.grid(row=0, column=1)
        question_type.set("single")
        question_type.bind("<<ComboboxSelected>>", lambda _: update_answer_ui())

        # Question Text
        tk.Label(editor, text="Question:").grid(row=1, column=0, sticky=tk.W)
        question_entry = tk.Text(editor, width=60, height=3)
        question_entry.grid(row=1, column=1, columnspan=2)

        # Choices
        choice_entries = []
        for i in range(4):
            tk.Label(editor, text=f"Choice {i+1}:").grid(row=2+i, column=0, sticky=tk.W)
            entry = tk.Text(editor, width=60, height=2)
            entry.grid(row=2+i, column=1, columnspan=2)
            choice_entries.append(entry)

        # Answer Selectors
        answer_frame = tk.Frame(editor)
        answer_frame.grid(row=6, column=0, columnspan=3, sticky=tk.W)

        # Single Answer
        single_answer_frame = tk.Frame(answer_frame)
        tk.Label(single_answer_frame, text="Correct Answer:").pack(side=tk.LEFT)
        single_answer_var = tk.IntVar()
        single_answer_combobox = ttk.Combobox(
            single_answer_frame, 
            textvariable=single_answer_var, 
            values=list(range(1, 5)),  # Display 1-4 to users
            state="readonly"
        )
        single_answer_combobox.pack(side=tk.LEFT)
        single_answer_combobox.current(0)

        # Multiple Answer
        multiple_answer_frame = tk.Frame(answer_frame)
        tk.Label(multiple_answer_frame, text="Correct Answers:").pack(side=tk.LEFT)
        multiple_answer_listbox = tk.Listbox(
            multiple_answer_frame, 
            selectmode=tk.MULTIPLE, 
            height=4,
            width=10
        )
        multiple_answer_listbox.pack(side=tk.LEFT)
        multiple_answer_listbox.insert(tk.END, "Choice 1", "Choice 2", "Choice 3", "Choice 4")

        # Difficulty
        tk.Label(editor, text="Difficulty (0.0-1.0):").grid(row=7, column=0, sticky=tk.W)
        difficulty_entry = tk.Entry(editor)
        difficulty_entry.grid(row=7, column=1, sticky=tk.W)

        # Load existing data
        if not new:
            index = self.question_listbox.curselection()[0]
            question = self.questions[index]
            question_type.set(question["type"])
            question_entry.insert("1.0", question["question"])
            for i, entry in enumerate(choice_entries):
                if i < len(question["choices"]):  # Ensure we don't access out-of-bounds indices
                    entry.insert("1.0", question["choices"][i])
                else:
                    entry.insert("1.0", "")  # Clear any unused choice fields
            difficulty_entry.insert(0, str(question["difficulty"]))
            
            if question["type"] == "single":
                single_answer_var.set(question["answer"] + 1)  # Show 1-4 to users
            else:
                for idx in question["answer"]:
                    multiple_answer_listbox.selection_set(idx)

        update_answer_ui()
        tk.Button(editor, text="Save", command=save).grid(row=8, column=0, columnspan=3)

    def delete_question(self):
        if not self.question_listbox.curselection():
            messagebox.showwarning("Warning", "No question selected.")
            return

        index = self.question_listbox.curselection()[0]
        self.questions.pop(index)
        self.question_listbox.delete(index)

    def save_json(self):
        if not self.file_path:
            self.file_path = filedialog.asksaveasfilename(defaultextension=".json", filetypes=[("JSON files", "*.json")])
        if not self.file_path:
            return

        try:
            with open(self.file_path, "w", encoding="utf-8") as file:
                json.dump(self.questions, file, indent=4)
            messagebox.showinfo("Success", "File saved successfully.")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to save file: {e}")

if __name__ == "__main__":
    root = tk.Tk()
    app = QuestionBankTool(root)
    root.mainloop()
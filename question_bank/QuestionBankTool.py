import csv
import tkinter as tk
from tkinter import filedialog, messagebox

class QuestionBankTool:
    def __init__(self, root):
        self.root = root
        self.root.title("Question Bank Tool")

        # UI Components
        self.file_label = tk.Label(root, text="No file selected")
        self.file_label.pack()

        self.create_button = tk.Button(root, text="Create New CSV", command=self.create_csv)
        self.create_button.pack()

        self.load_button = tk.Button(root, text="Load CSV", command=self.load_csv)
        self.load_button.pack()

        self.question_listbox = tk.Listbox(root, width=100, height=15)
        self.question_listbox.pack()

        self.add_button = tk.Button(root, text="Add Question", command=self.add_question)
        self.add_button.pack()

        self.edit_button = tk.Button(root, text="Edit Selected Question", command=self.edit_question)
        self.edit_button.pack()

        self.delete_button = tk.Button(root, text="Delete Selected Question", command=self.delete_question)
        self.delete_button.pack()

        self.save_button = tk.Button(root, text="Save CSV", command=self.save_csv)
        self.save_button.pack()

        self.questions = []
        self.file_path = None

    def create_csv(self):
        self.file_path = filedialog.asksaveasfilename(defaultextension=".csv", filetypes=[("CSV files", "*.csv")])
        if not self.file_path:
            return

        try:
            with open(self.file_path, "w", newline='', encoding="utf-8") as file:
                fieldnames = ["Question", "Choice1", "Choice2", "Choice3", "Choice4", "Answer", "Difficulty"]
                writer = csv.DictWriter(file, fieldnames=fieldnames)
                writer.writeheader()
            self.file_label.config(text=f"Created: {self.file_path}")
            self.questions.clear()
            self.question_listbox.delete(0, tk.END)
            messagebox.showinfo("Success", "New CSV file created successfully.")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to create file: {e}")

    def load_csv(self):
        self.file_path = filedialog.askopenfilename(filetypes=[("CSV files", "*.csv")])
        if not self.file_path:
            return

        self.questions.clear()
        self.question_listbox.delete(0, tk.END)

        try:
            with open(self.file_path, "r", newline='', encoding="utf-8") as file:
                reader = csv.DictReader(file)
                for row in reader:
                    self.questions.append(row)
                    self.question_listbox.insert(tk.END, row["Question"])
            self.file_label.config(text=f"Loaded: {self.file_path}")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to load file: {e}")

    def add_question(self):
        self.edit_question(new=True)

    def edit_question(self, new=False):
        def save():
            question = {
                "Question": question_entry.get(),
                "Choice1": choice1_entry.get(),
                "Choice2": choice2_entry.get(),
                "Choice3": choice3_entry.get(),
                "Choice4": choice4_entry.get(),
                "Answer": answer_entry.get(),
                "Difficulty": difficulty_entry.get()
            }
            if new:
                self.questions.append(question)
                self.question_listbox.insert(tk.END, question["Question"])
            else:
                index = self.question_listbox.curselection()[0]
                self.questions[index] = question
                self.question_listbox.delete(index)
                self.question_listbox.insert(index, question["Question"])
            editor.destroy()

        if not new and not self.question_listbox.curselection():
            messagebox.showwarning("Warning", "No question selected.")
            return

        editor = tk.Toplevel(self.root)
        editor.title("Edit Question")

        tk.Label(editor, text="Question:").grid(row=0, column=0, sticky=tk.W)
        question_entry = tk.Entry(editor, width=50)
        question_entry.grid(row=0, column=1)

        tk.Label(editor, text="Choice 1:").grid(row=1, column=0, sticky=tk.W)
        choice1_entry = tk.Entry(editor, width=50)
        choice1_entry.grid(row=1, column=1)

        tk.Label(editor, text="Choice 2:").grid(row=2, column=0, sticky=tk.W)
        choice2_entry = tk.Entry(editor, width=50)
        choice2_entry.grid(row=2, column=1)

        tk.Label(editor, text="Choice 3:").grid(row=3, column=0, sticky=tk.W)
        choice3_entry = tk.Entry(editor, width=50)
        choice3_entry.grid(row=3, column=1)

        tk.Label(editor, text="Choice 4:").grid(row=4, column=0, sticky=tk.W)
        choice4_entry = tk.Entry(editor, width=50)
        choice4_entry.grid(row=4, column=1)

        tk.Label(editor, text="Answer:").grid(row=5, column=0, sticky=tk.W)
        answer_entry = tk.Entry(editor, width=50)
        answer_entry.grid(row=5, column=1)

        tk.Label(editor, text="Difficulty (0.000 - 1.000):").grid(row=6, column=0, sticky=tk.W)
        difficulty_entry = tk.Entry(editor, width=50)
        difficulty_entry.grid(row=6, column=1)

        if not new:
            index = self.question_listbox.curselection()[0]
            question = self.questions[index]
            question_entry.insert(0, question["Question"])
            choice1_entry.insert(0, question["Choice1"])
            choice2_entry.insert(0, question["Choice2"])
            choice3_entry.insert(0, question["Choice3"])
            choice4_entry.insert(0, question["Choice4"])
            answer_entry.insert(0, question["Answer"])
            difficulty_entry.insert(0, question["Difficulty"])

        tk.Button(editor, text="Save", command=save).grid(row=7, column=0, columnspan=2)

    def delete_question(self):
        if not self.question_listbox.curselection():
            messagebox.showwarning("Warning", "No question selected.")
            return

        index = self.question_listbox.curselection()[0]
        self.questions.pop(index)
        self.question_listbox.delete(index)

    def save_csv(self):
        if not self.file_path:
            self.file_path = filedialog.asksaveasfilename(defaultextension=".csv", filetypes=[("CSV files", "*.csv")])

        if not self.file_path:
            return

        try:
            with open(self.file_path, "w", newline='', encoding="utf-8") as file:
                fieldnames = ["Question", "Choice1", "Choice2", "Choice3", "Choice4", "Answer", "Difficulty"]
                writer = csv.DictWriter(file, fieldnames=fieldnames)
                writer.writeheader()
                writer.writerows(self.questions)
            messagebox.showinfo("Success", "File saved successfully.")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to save file: {e}")

if __name__ == "__main__":
    root = tk.Tk()
    app = QuestionBankTool(root)
    root.mainloop()

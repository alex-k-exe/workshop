# -----Statement of Authorship----------------------------------------#
#
# This is an individual assessment task for QUT's teaching unit
# IFB104, "Building IT Systems", Semester 2, 2024. By submitting
# this code I agree that it represents my own work. I am aware of
# the University rule that a student must not act in a manner
# which constitutes academic dishonesty as stated and explained
# in QUT's Manual of Policies and Procedures, Section C/5.3
# "Academic Integrity" and Section E/2.1 "Student Code of Conduct".
#
# Put your student number here as an integer and your name as a
# character string:
#
student_number = 12370002
student_name = "Alex Kammin"
#
# NB: All files submitted for this assessable task will be subjected
# to automated plagiarism analysis using a tool such as the Measure
# of Software Similarity (http://theory.stanford.edu/~aiken/moss/).
#
# --------------------------------------------------------------------#

# -----Assessment Task 1 Description----------------------------------#
#
# This assessment task tests your skills at processing large data
# sets, creating reusable code and following instructions to retrieve
# information. The incomplete Python program below is
# missing a crucial function. You are required to complete this
# function so that when the program runs it interacts with the user
# to perform a range of functionalities.
#  See the instructions in part A and part B for full details.
#
# Note that this assessable assignment is in multiple parts,
# simulating incremental release of instructions by a paying
# "client". This single template file will be used for all parts
# and you will submit your final solution as this single Python 3
# file only, whether or not you complete all requirements for the
# assignment.
#
# This file relies on one other Python modules but all of your code
# must appear in this file only. You may not change any of the code
# in the other module and you should not submit the other module
# with your solution. The markers will use their own copies of the
# other module to test your code, so your solution will not work
# if it relies on changes made to any other files.
#
# --------------------------------------------------------------------#

# -----Preamble-------------------------------------------------------#
#
# This section imports necessary functions used to execute your code.
# You must NOT change any of the code in this section, and you may
# NOT import any non-standard Python modules that need to be
# downloaded and installed separately

# Import standard Python modules needed to complete this assignment.
# You should not need to use any other modules for your solution.
# In particular, your solution must NOT rely on any non-standard
# Python modules that need to be downloaded and installed separately,
# because the markers will not have access to such modules.
from sys import exit as abort


# Confirm that the student has declared their authorship
if not isinstance(student_number, int):
    print(
        "\nUnable to run: No student number supplied",
        "(must be an integer), aborting!\n",
    )
    abort()
if not isinstance(student_name, str):
    print(
        "\nUnable to run: No student name supplied",
        "(must be a character string), aborting!\n",
    )
    abort()


# -----Student's Solution---------------------------------------------#
#
# Complete the assignment by replacing the dummy function below with
# your own function and any other functions needed to support it.
# All of your solution code must appear in this section. Do NOT put
# any of your code in any other sections and do NOT change any of
# the provided code except as allowed by the comments in the next
# section.
#

# All of your code goes in, or is called from, this function.

SAMPLE_BOOKLIST = '1. the art of coding: mastering the craft of software development. 2. journey through time: a historical exploration of ancient civilizations. 3. mindful living: techniques for a balanced and peaceful life. 4. the science of success: strategies for achieving your goals. 4. creative writing: unlocking your imagination and storytelling skills. 5. healthy eating: a guide to nutritious and delicious meals.'

import re
import doctest

def make_book_list(booklist: str) -> list[list[str]]:
	"""
	Returns a list of lists that stores the book titles and subtitles

	Test correctly formatted booklist string
	>>> make_book_list(SAMPLE_BOOKLIST)
	[['the art of coding', 'mastering the craft of software development'], ['journey through time', 'a historical exploration of ancient civilizations'], ['mindful living', 'techniques for a balanced and peaceful life'], ['the science of success', 'strategies for achieving your goals'], ['creative writing', 'unlocking your imagination and storytelling skills'], ['healthy eating', 'a guide to nutritious and delicious meals']]

	Test partially correctly formatted booklist string
	>>> make_book_list('1. the art of codthe craft of software development. 2. journey through time: a historical exploration of ancient civilizations. mindful living: techniques for a balanced and peaceful life.)
	[["journey through time", "a historical exploration of ancient civilizations."]]
	"""

	# (?:\d\.\s) - matches (but doesn't capture) a number, ".", and whitespace
	# ([a-zA-Z\s:]+) - captures the title and subtitle
	# e.g. for first book it will capture "the art of coding: mastering the craft of software development"
	books: list[str] = re.findall(r"(?:\d\.\s)([a-zA-Z\s:]+)", booklist)
	print('kkkkkk', books)
	return [book.split(": ") for book in books]

def make_index(booklist: list[list[str]], titles_or_subtitles: int):
	"""
	Test titles
	>>> make_index(make_book_list(SAMPLE_BOOKLIST), 0)
	[['the', [0, 3]], ['art', [0]], ['of', [0, 3]], ['coding', [0]], ['journey', [1]], ['through', [1]], ['time', [1]], ['mindful', [2]], ['living', [2]], ['the', [0, 3]], ['science', [3]], ['of', [0, 3]], ['success', [3]], ['creative', [4]], ['writing', [4]], ['healthy', [5]], ['eating', [5]]]

	Test subtitles
	>>> make_index(make_book_list(SAMPLE_BOOKLIST), 1)
	[['mastering', [0]], ['the', [0]], ['craft', [0]], ['of', [0, 1]], ['software', [0]], ['development', [0]], ['a', [0, 1, 2, 3, 4, 5]], ['historical', [1]], ['exploration', [1]], ['of', [0, 1]], ['ancient', [1]], ['civilizations', [1]], ['techniques', [2]], ['for', [2, 3]], ['a', [0, 1, 2, 3, 4, 5]], ['balanced', [2]], ['and', [2, 4, 5]], ['peaceful', [2]], ['life', [2]], ['strategies', [3]], ['for', [2, 3]], ['achieving', [3]], ['your', [3, 4]], ['goals', [3]], ['unlocking', [4]], ['your', [3, 4]], ['imagination', [4]], ['and', [2, 4, 5]], ['storytelling', [4]], ['skills', [4]], ['a', [0, 1, 2, 3, 4, 5]], ['guide', [5]], ['to', [1, 4, 5]], ['nutritious', [5]], ['and', [2, 4, 5]], ['delicious', [5]], ['meals', [5]]]
	"""

	if titles_or_subtitles != 1:
		titles_or_subtitles = 0

	index = []

	for book in booklist:
		words = book[titles_or_subtitles].lower().split(" ")
		for word in words:
			if not index.__contains__(word):
				index.append([word, []])
			for (i, book2) in enumerate(booklist):
				if book2[titles_or_subtitles].__contains__(word):
					index[index.__len__() - 1][1].append(i)

	return index

def title_length(booklist: list[list[str]]) -> float:
	"""
	Takes the “booklist” and returns the average length (in number of words) of the book titles and subtitles.
	"""
	total_words = 0

	for book in booklist:
		# There is a typo in the specifications saying that title_length "returns the average length (in number of words) of the book titles"
		# But it actually means the titles and subtitles
		total_words += (book[0] + " " + book[1]).split(" ").__len__()

	if booklist.__len__() == 0:
		return 0
	else:
		return total_words / booklist.__len__()

def search_word(word: str, index_subtitles: list[list[str | list[int]]], booklist: list[list[str]]) -> str:
	"""
	Takes the “booklist” and a “word”, and returns the titles of all the books containing the word in their subtitle, separated by “;”.
	"""
	result = ""
	for book in booklist:
		if not book[1].__contains__(word):
			continue
		title_words = book[0].split(" ")

		# Capitalise the first word even if it's not important
		result += title_words[0][0].upper() + title_words[0][1:] + " "
		# Capitalise each word after the first if it's important
		for title_word in title_words[1:]:
			unimportant_words = ["the", "of", "and", "a", "an"]
			if unimportant_words.__contains__(title_word):
				result += title_word + " "
			else:
				result += title_word[0].upper() + title_word[1:] + " "
		result = result[:result.__len__() - 1] + ";"
	if result.__len__() == 0:
		return "no book contains " + word
	else:
		result = result[:result.__len__() - 1]

	return re.sub(r"Power", "Superpower", result)

def word_occurences(word: str, index_subtitles: list[list[str | list[int]]], booklist: list[list[str]]) -> int:
	"""
	Takes the “booklist” and a “word”, and returns the number of times the word appears in the booklist.
	Assuming that the word can appear within other words
	"""
	count = 0
	for book in booklist:
		for title_word in book[0].split(" ") + book[1].split(" "):
			if title_word == word:
				count += 1
	return count

# Within the interact function, as per the examples below. Write the python code to ask a user to enter the text, then ask them which action they want to perform.
# If the text initially entered by the user is less than 50 characters long, they should be prompted to enter a longer text.
# If they want to search for a word or count its occurrences, then prompt them to enter a word, then call the appropriate function and print the outcome.

def interact():
	booklist: list[list[str]] = []
	while True:
		temp = input("Please enter your booklist:")
		if temp.__len__() < 50:
			print("Your booklist is too short, please enter at least 50 characters")
		else:
			booklist = make_book_list(temp)
			break

	while True:
		print("Choose an action:\n" + "1. Get average title length\n" + "2. Search for a word in subtitles\n" + "3. Count word occurrences in titles and subtitles")

		action: int = 0
		while True:
			try:
				action = int(input("Enter your choice (1-3):"))
			except ValueError:
				continue

			if action > 0 and action < 4:
				break

		formatted_booklist = make_index(booklist, 0)
		print("-------------")
		if action == 1:
			print("Average title length: ", title_length(booklist))
		elif action == 2:
			word = input("Enter a word to search: ")
			print("Matching titles: ", search_word(word, formatted_booklist, booklist))
		else:
			word = input("Enter a word to count occurrences: ")
			print("Word occurrences: ", word_occurences(word, formatted_booklist, booklist))
		print("-------------")


#-----Main Program to Run Student's Solution-----#
#You must NOT change any of the code in this section.

if __name__ == "__main__":
	doctest.testmod()
	interact()

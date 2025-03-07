#-----Statement of Authorship----------------------------------------#
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
#--------------------------------------------------------------------#

#-----Assessment Task 1 Description----------------------------------#
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
#--------------------------------------------------------------------#

#-----Preamble-------------------------------------------------------#
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
	print('\nUnable to run: No student number supplied',
	'(must be an integer), aborting!\n')
	abort()
if not isinstance(student_name, str):
	print('\nUnable to run: No student name supplied',
	'(must be a character string), aborting!\n')
	abort()




#-----Student's Solution---------------------------------------------#
#
# Complete the assignment by replacing the dummy function below with
# your own function and any other functions needed to support it.
# All of your solution code must appear in this section. Do NOT put
# any of your code in any other sections and do NOT change any of
# the provided code except as allowed by the comments in the next
# section.
#

# All of your code goes in, or is called from, this function.

import re

def title_length(booklist: str):
	"""
	Takes the “booklist” and returns the average length (in number of words) of the book titles.
	"""
	# (?:\d\.\s) - matches (but doesn't capture) a number, ".", and whitespace
	# ([a-zA-Z\s]+) - captures the title (but not the subtitle)
	titles = re.findall(r'(?:\d\.\s)([a-zA-Z\s]+)', booklist)
	total_words = 0

	for title in titles:
		total_words += title.split(" ").__len__()

	if titles.__len__() == 0:
		return 0
	else:
		return total_words / titles.__len__()

def search_word(booklist: str, word: str):
	"""
	Takes the “booklist” and a “word”, and returns the titles of all the books containing the word in their subtitle, separated by “;”.
	Assuming that the word can appear within other words
	"""
	# (?:\d\.\s) - matches (but doesn't capture) a number, ".", and whitespace
	# ([a-zA-Z\s:]+) - captures the title and subtitle
	books: list[str] = re.findall(r'(?:\d\.\s)([a-zA-Z\s:]+)', booklist)

	result = ""
	for book in books:
		separated = book.split(': ')
		if separated[1].__contains__(word):
			result += separated[0] + ";"

	return result[:result.__len__() - 1]

def word_occurences(booklist: str, word: str):
	"""
	Takes the “booklist” and a “word”, and returns the number of times the word appears in the booklist.
	Assuming that the word can appear within other words
	"""
	return booklist.count(word)

# Within the interact function, as per the examples below. Write the python code to ask a user to enter the text, then ask them which action they want to perform. If the text initially entered by the user is less than 50 characters long, they should be prompted to enter a longer text. If they want to search for a word or count its occurrences, then prompt them to enter a word, then call the appropriate function and print the outcome.

def interact():
	booklist = ""
	while True:
		booklist = input("Please enter your booklist:")
		if booklist.__len__() < 50:
			print("Your booklist is too short, please enter at least 50 characters")
		else:
			break

	print("Choose an action:\n" + "1. Get average title length\n" + "2. Search for a word in subtitles\n" + "3. Count word occurrences in titles and subtitles")

	action: int = 0
	while True:
		try:
			action = int(input("Enter your choice (1-3):"))
		except ValueError:
			continue

		if action > 0 and action < 4:
			break

	if action == 1:
		print("Average title length: ", title_length(booklist))
	elif action == 2:
		word = input("Enter a word to search: ")
		print("Matching titles: ", search_word(booklist, word))
	else:
		word = input("Enter a word to count occurrences: ")
		print("Word occurrences: ", word_occurences(booklist, word))


#-----Main Program to Run Student's Solution-------------------------#
#You must NOT change any of the code in this section.
if __name__ == "__main__":
    interact()


















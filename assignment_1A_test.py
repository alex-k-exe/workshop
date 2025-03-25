import unittest
from assignment_1A import make_book_list, make_index, title_length, search_word, word_occurences

formatted_booklist = [['the art of coding', 'mastering the craft of software development'], ['journey through time', 'a historical exploration of ancient civilizations'], ['mindful living', 'techniques for a balanced and peaceful life'], ['the science of success', 'strategies for achieving your goals'], ['creative writing', 'unlocking your imagination and storytelling skills'], ['healthy eating', 'a guide to nutritious and delicious meals'], ['financial freedom', 'steps to achieve economic independence'], ['the world of art', 'understanding and appreciating visual masterpieces'], ['digital marketing', 'tactics for building an online presence'], ['the power of habit', 'transforming your life one step at a time']]

class TestTask1(unittest.TestCase):
	def test_make_booklist(self):
		sample_booklist = '1. the art of coding: mastering the craft of software development. 2. journey through time: a historical exploration of ancient civilizations. 3. mindful living: techniques for a balanced and peaceful life. 4. the science of success: strategies for achieving your goals. 4. creative writing: unlocking your imagination and storytelling skills. 5. healthy eating: a guide to nutritious and delicious meals. 6. financial freedom: steps to achieve economic independence. 7. the world of art: understanding and appreciating visual masterpieces. 7. digital marketing: tactics for building an online presence. 9. the power of habit: transforming your life one step at a time. '

		self.assertEqual(make_book_list(sample_booklist), formatted_booklist)

	def test_make_index(self):
		pass

	def test_title_length(self):
		self.assertEqual(title_length(formatted_booklist), 9.0)

	def test_search_word(self):
		self.assertEqual(search_word("life", make_index(formatted_booklist), formatted_booklist), "Mindful Living;The Superpower of Habit")

	def test_word_occurences(self):
		self.assertEqual(word_occurences("life", make_index(formatted_booklist), formatted_booklist), 2)

if __name__ == "__main__":
    unittest.main()

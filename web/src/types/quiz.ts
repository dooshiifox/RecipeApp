import type { Formattable } from '.';

/** The quiz that appears at the end of a recipe. */
export interface Quiz {
	/** The questions on the quiz. */
	questions: Question[];
	/** The experience reward for getting all questions correct. */
	allCorrectReward: number;
}

/** A question on a recipe's Quiz. */
export interface Question {
	/** The question itself. */
	question: Formattable;
	/** The assisting information to the question, if necessary. */
	description?: Formattable;
	/** An image displayed alongside the question. */
	image?: string;
	/** A list of correct answers. Should be short, and must have >= 1. */
	correctAnswers: Formattable[];
	/** A list of incorrect answers. Should be short. Can be empty. */
	wrongAnswers: Formattable[];
	/** The experience reward for getting the question correct. */
	reward: number;
}

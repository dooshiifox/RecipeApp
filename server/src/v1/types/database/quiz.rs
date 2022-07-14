use crate::v1::types::*;

/// A Quiz stores information about the quizzes at the end of recipes.
/// They are used by the [`Recipe`] type.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Quiz {
    /// The questions present in the quiz
    questions: Vec<Question>,
    /// The reward for getting all questions correct.
    all_correct_reward: u16,
}

impl Quiz {
    /// Constructs a new Quiz.
    pub fn new(all_correct_reward: u16) -> Self {
        Self {
            questions: vec![],
            all_correct_reward,
        }
    }

    /// Adds a [`Question`] to the Quiz.
    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.push(question);
        self
    }
}

/// A Question stores information about a single question presented
/// at the end of recipes. They are stored in a [`Quiz`].
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    /// The question title.
    question: Formattable,
    /// The question description.
    description: Option<Formattable>,
    /// An image associated with the question.
    image: Option<Url>,
    /// The correct answer(s). Max 400 chars.
    correct_answers: Vec<Formattable>,
    /// The incorrect answer(s). Max 400 chars.
    wrong_answers: Vec<Formattable>,
    /// The reward for getting this question correct.
    reward: u16,
}

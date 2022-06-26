use crate::v1::types::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Quiz {
    /// The questions present in the quiz
    questions: Vec<Question>,
    /// The reward for getting all questions correct.
    all_correct_reward: u16,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Question {
    /// The question itself. Max 400 chars.
    question: Vec<Formattable>,
    /// An image associated with the question.
    image: Option<Url>,
    /// The correct answer(s). Max 400 chars.
    correct_answers: Vec<Formattable>,
    /// The incorrect answer(s). Max 400 chars.
    wrong_answers: Vec<Formattable>,
    /// The reward for getting this question correct.
    reward: u16,
}

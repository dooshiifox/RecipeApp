use crate::v1::types::*;

/// A Quiz stores information about the quizzes at the end of recipes.
/// They are used by the [`Recipe`] type.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
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

impl Question {
    /// Creates a [`QuestionBuilder`] used to create new Questions.
    pub fn builder() -> QuestionBuilder {
        QuestionBuilder::default()
    }
}

/// A builder for the [`Question`] type.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Default)]
pub struct QuestionBuilder {
    /// The question title.
    question: Option<Formattable>,
    /// The question description.
    description: Option<Formattable>,
    /// An image associated with the question.
    image: Option<Url>,
    /// The correct answer(s). Max 400 chars.
    correct_answers: Vec<Formattable>,
    /// The incorrect answer(s). Max 400 chars.
    wrong_answers: Vec<Formattable>,
    /// The reward for getting this question correct.
    reward: Option<u16>,
}

impl QuestionBuilder {
    /// Builds a [`Question`] from the builder.
    pub fn build(self) -> Result<Question, String> {
        if self.correct_answers.is_empty() {
            return Err("Must be at least one correct answer.".to_string());
        }

        if self.correct_answers.len() + self.wrong_answers.len() > 8 {
            return Err("Cannot be more than 8 total answers.".to_string());
        }

        Ok(Question {
            question: self
                .question
                .ok_or_else(|| "Question title was not set".to_string())?,
            description: self.description,
            image: self.image,
            correct_answers: self.correct_answers,
            wrong_answers: self.wrong_answers,
            reward: self
                .reward
                .ok_or_else(|| "No reward field set on question.".to_string())?,
        })
    }

    /// Set the title on the question.
    pub fn title(mut self, title: impl ToString) -> Self {
        self.question = Some(Formattable::new(title));
        self
    }

    /// Set the description on the question.
    pub fn description(mut self, description: impl ToString) -> Self {
        self.description = Some(Formattable::new(description));
        self
    }

    /// Sets the image on the question.
    pub fn image(mut self, image: impl ToString) -> Self {
        self.image = Some(Url::new(image));
        self
    }

    /// Adds a new correct answer to a question.
    pub fn add_correct(mut self, answer: impl ToString) -> Self {
        self.correct_answers.push(Formattable::new(answer));
        self
    }

    /// Adds a new wrong answer to a question.
    pub fn add_wrong(mut self, answer: impl ToString) -> Self {
        self.wrong_answers.push(Formattable::new(answer));
        self
    }

    /// Sets the reward for getting this question correct.
    pub fn reward(mut self, reward: u16) -> Self {
        self.reward = Some(reward);
        self
    }
}

use  crate::{
    models::{Answer, AnswerDetail, DBError, QuestionId}, persistance::answers_dao::AnswersDao
};

#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest(String),
    InternalServerError(String),
}

impl HandlerError {
    pub fn default_internal_error() -> Self {
        HandlerError::InternalServerError("Something went wrong! Please try again.".to_owned())
    }
}

pub async fn create_answer(answer: Answer, answers_dao: &Box<dyn AnswersDao + Sync + Send>) ->  Result<AnswerDetail, HandlerError>{

    let answer = answers_dao.create_answer(answer).await;
    match answer {
        Ok(answer) => Ok(answer),
        Err(err) => {
            error!("{:?}", err);

            match err {
                DBError::InvalidUUID(s) => Err(HandlerError::BadRequest(s)),
                _ => Err(HandlerError::default_internal_error()),
            }
        }
    }
}

pub async fn get_answers(question_uuid: QuestionId, answers_dao: &Box<dyn AnswersDao + Sync + Send>) -> Result<Vec<AnswerDetail>, HandlerError>{
    let answers = answers_dao.get_answers(question_uuid.question_uuid).await;

    match answers {
        Ok(answers) => Ok(answers),
        Err(e) => {
            error!("{:?}", e);
            Err(HandlerError::default_internal_error())
        }

    }


}
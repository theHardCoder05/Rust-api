use rocket::{serde::json::Json, State};

use crate::{
    models::*,
    persistance::{answers_dao::AnswersDao},
};
mod handlers_inner;

use handlers_inner::*;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalServerError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s)=> Self::BadRequest(s),
            HandlerError::InternalServerError(s)=> Self::InternalServerError(s),
        }
    }
}

#[post("/answer", data= "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answer_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Result<Json<AnswerDetail>, APIError> {
    match handlers_inner::create_answer(answer.0, answer_dao.inner()).await{
        Ok(q) => Ok(Json(q)),
        Err(err) => Err(err.into()),
    }
}

#[get("/answers", data = "<question_uuid>")]
pub async fn get_answers(question_uuid:Json<QuestionId>,
                        answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
                    ) -> Result<Json<Vec<AnswerDetail>>, APIError> {
                        match handlers_inner::get_answers(question_uuid.0, answers_dao.inner()).await {
                            Ok(q)=> Ok(Json(q)),
                            Err(err) => Err(err.into()),
                    }
                }
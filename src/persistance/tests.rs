mod answers_tests {
    use sqlx::PgPool;

    use crate::{
        models::{Answer, DBError},
        persistance::{
            answers_dao::{AnswersDao, AnswersDaoImpl},
        },
    };

    #[sqlx::test]
    async fn create_answer_should_succeed(pool: PgPool) -> Result<(), String> {
    
        let answer_doa = AnswersDaoImpl::new(pool);

        let result = answer_doa
            .create_answer(Answer {
                question_uuid: "12345".to_owned(),
                content: "test content".to_owned(),
            })
            .await
            .map_err(|e| format!("{:?}", e))?;

        if result.content != "test contnt".to_owned() {
            return Err("Incorrect answer content".to_owned());
        }

        Ok(())
    }
}
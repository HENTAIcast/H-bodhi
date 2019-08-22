use serde::{Deserialize, Serialize};

use crate::error::{QueryError, BodhiError};
use crate::query::csrf::CSRFQuery;
use crate::service::BodhiService;

#[derive(Debug, Clone)]
pub enum Karma {
    Positive,
    Neutral,
    Negative,
}

impl Into<i32> for Karma {
    fn into(self) -> i32 {
        match self {
            Karma::Positive => 1,
            Karma::Neutral => 0,
            Karma::Negative => -1,
        }
    }
}

impl Into<String> for Karma {
    fn into(self) -> String {
        match self {
            Karma::Positive => String::from("+1"),
            Karma::Neutral => String::from("0"),
            Karma::Negative => String::from("-1"),
        }
    }
}

#[derive(Debug, Serialize)]
struct CommentData {
    update: String,
    text: String,
    karma: i32,
    karma_critpath: Option<i32>,
    bug_feedback: Option<Vec<i32>>,
    testcase_feedback: Option<Vec<i32>>,
    csrf_token: String,
}

#[derive(Debug, Deserialize)]
pub struct NewComment {
    comment: u32,
    caveats: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct CommentBuilder {
    update: String,
    text: Option<String>,
    karma: Option<Karma>,
    karma_critpath: Option<Karma>,
    bug_feedback: Option<Vec<Karma>>,
    testcase_feedback: Option<Vec<Karma>>,
}

impl CommentBuilder {
    pub fn new(update: String) -> Self {
        CommentBuilder {
            update,
            text: None,
            karma: None,
            karma_critpath: None,
            bug_feedback: None,
            testcase_feedback: None,
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn karma(mut self, karma: Karma) -> Self {
        self.karma = Some(karma);
        self
    }

    pub fn karma_critpath(mut self, karma: Karma) -> Self {
        self.karma_critpath = Some(karma);
        self
    }

    pub fn bug_feedback(mut self, feedback: Vec<Karma>) -> Self {
        self.bug_feedback = Some(feedback);
        self
    }

    pub fn testcase_feedback(mut self, feedback: Vec<Karma>) -> Self {
        self.testcase_feedback = Some(feedback);
        self
    }

    pub fn create(self, bodhi: &BodhiService) -> Result<NewComment, QueryError> {
        // let user = bodhi.username()?;
        bodhi.username()?;

        // TODO: check if lengths of feedback vectors is good
        let path = String::from("/comments/");

        let csrf_token = CSRFQuery::new().query(&bodhi)?;

        let text = match self.text {
            Some(text) => text,
            None => String::from(""),
        };

        let karma: i32 = match self.karma {
            Some(karma) => karma.into(),
            None => 0,
        };

        let karma_critpath: Option<i32> = match self.karma_critpath {
            Some(karma) => Some(karma.into()),
            None => None,
        };

        let bug_feedback: Option<Vec<i32>> = match self.bug_feedback {
            Some(feedback) => Some(feedback.into_iter().map(|k| k.into()).collect()),
            None => None,
        };

        let testcase_feedback: Option<Vec<i32>> = match self.testcase_feedback {
            Some(feedback) => Some(feedback.into_iter().map(|k| k.into()).collect()),
            None => None,
        };

        let comment = CommentData {
            update: self.update,
            text,
            karma,
            karma_critpath,
            bug_feedback,
            testcase_feedback,
            csrf_token,
        };

        let data = match serde_json::to_string(&comment) {
            Ok(data) => data,
            Err(error) => return Err(QueryError::SerializationError { error }),
        };

        // FIXME

        /*
        {\"status\": \"error\",
         \"errors\": [{
              \"location\": \"body\",
              \"name\": \"comment\",
              \"description\": \"You must provide a comment author\"
              }]
        }
        */

        let mut response = bodhi.post(&path, data, None)?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().unwrap_or_else(|_| String::from(""));

            let error: BodhiError = serde_json::from_str(&text)?;
            return Err(QueryError::BodhiError { error });
        };

        let result = response.text()?;
        let comment: NewComment = serde_json::from_str(&result)?;

        Ok(comment)
    }
}

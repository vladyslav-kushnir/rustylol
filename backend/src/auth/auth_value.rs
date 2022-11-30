use super::error::Error;

#[derive(Debug)]
pub enum AuthValue {
    Bearer { token: String, roles: Vec<String> }
}

pub enum AuthHeader {
    Bearer { token: String }
}

impl TryFrom<Option<String>> for AuthHeader {
    type Error = Error;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => Self::try_from(Some(s.as_str())),
            None => Err(Error::MissingAuthHeader)
        }
    }
}

impl TryFrom<Option<&str>> for AuthHeader {
    type Error = Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        if let Some(str_value) = value {
            let parts: Vec<&str> = str_value.split(' ').collect();

            return match parts[..] {
                ["Bearer", token] => {
                    Ok(Self::Bearer {
                        token: token.to_string()
                    })
                },
                _ => Err(Error::InvalidAuthHeader)
            }
        }

        Err(Error::MissingAuthHeader)
    }
}
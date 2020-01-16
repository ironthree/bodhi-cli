use std::convert::TryFrom;

use bodhi::*;

// TODO: upstream generic implementations of these to bodhi-rs

pub fn str_to_compose_request(request: &str) -> Result<ComposeRequest, String> {
    match request {
        "stable" => Ok(ComposeRequest::Stable),
        "testing" => Ok(ComposeRequest::Testing),
        _ => Err(format!("Not a recognised value for compose request: {}", request)),
    }
}

pub fn op_str_to_op_content_type(op_string: Option<&String>) -> Result<Option<ContentType>, String> {
    match op_string {
        Some(string) => match string.to_lowercase().as_str() {
            "rpm" => Ok(Some(ContentType::RPM)),
            "module" => Ok(Some(ContentType::Module)),
            "flatpak" => Ok(Some(ContentType::Flatpak)),
            "container" => Ok(Some(ContentType::Container)),
            _ => Err(format!("Not a recognised content type: {}", string)),
        },
        None => Ok(None),
    }
}

pub fn op_str_to_op_date(op_string: Option<&String>) -> Result<Option<BodhiDate>, String> {
    match op_string {
        Some(date) => match BodhiDate::try_from(date.as_str()) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Err(format!("Date in invalid format: {}", date)),
        },
        None => Ok(None),
    }
}

pub fn op_str_to_op_karma(op_string: Option<&String>) -> Result<Option<Karma>, String> {
    match op_string {
        Some(string) => match string.as_str() {
            "1" | "+1" => Ok(Some(Karma::Positive)),
            "0" => Ok(Some(Karma::Neutral)),
            "-1" => Ok(Some(Karma::Negative)),
            _ => Err(format!("Not a recognised value for karma: {}", string)),
        },
        None => Ok(None),
    }
}

pub fn op_str_to_op_severity(op_string: Option<&String>) -> Result<Option<UpdateSeverity>, String> {
    match op_string {
        Some(string) => match string.to_lowercase().as_str() {
            "unspecified" => Ok(Some(UpdateSeverity::Unspecified)),
            "low" => Ok(Some(UpdateSeverity::Low)),
            "medium" => Ok(Some(UpdateSeverity::Medium)),
            "high" => Ok(Some(UpdateSeverity::High)),
            "urgent" => Ok(Some(UpdateSeverity::Urgent)),
            _ => Err(format!("Not a recognised value for severity: {}", string)),
        },
        None => Ok(None),
    }
}

pub fn op_str_to_op_update_request(op_string: Option<&String>) -> Result<Option<UpdateRequest>, String> {
    match op_string {
        Some(string) => match string.to_lowercase().as_str() {
            "obsolete" => Ok(Some(UpdateRequest::Obsolete)),
            "revoke" => Ok(Some(UpdateRequest::Revoke)),
            "stable" => Ok(Some(UpdateRequest::Stable)),
            "testing" => Ok(Some(UpdateRequest::Testing)),
            "unpush" => Ok(Some(UpdateRequest::Unpush)),
            _ => Err(format!("Not a recognised value for update request: {}", string)),
        },
        None => Ok(None),
    }
}

pub fn op_str_to_op_update_status(op_string: Option<&String>) -> Result<Option<UpdateStatus>, String> {
    match op_string {
        Some(string) => match string.to_lowercase().as_str() {
            "obsolete" => Ok(Some(UpdateStatus::Obsolete)),
            "pending" => Ok(Some(UpdateStatus::Pending)),
            "side_tag_active" => Ok(Some(UpdateStatus::SideTagActive)),
            "side_tag_expired" => Ok(Some(UpdateStatus::SideTagExpired)),
            "stable" => Ok(Some(UpdateStatus::Stable)),
            "testing" => Ok(Some(UpdateStatus::Testing)),
            "unpushed" => Ok(Some(UpdateStatus::Unpushed)),
            _ => Err(format!("Not a recognised value for status: {}", string)),
        },
        None => Ok(None),
    }
}

pub fn op_str_to_op_update_suggestion(op_string: Option<&String>) -> Result<Option<UpdateSuggestion>, String> {
    match op_string {
        Some(string) => match string.to_lowercase().as_str() {
            "unspecified" => Ok(Some(UpdateSuggestion::Unspecified)),
            "reboot" => Ok(Some(UpdateSuggestion::Reboot)),
            "logout" => Ok(Some(UpdateSuggestion::Logout)),
            _ => Err(format!("Not a recognised value for suggestion: {}", string)),
        },
        None => Ok(None),
    }
}

pub fn op_str_to_op_update_type(op_string: Option<&String>) -> Result<Option<UpdateType>, String> {
    match op_string {
        Some(string) => match string.to_lowercase().as_str() {
            "unspecified" => Ok(Some(UpdateType::Unspecified)),
            "enhancement" => Ok(Some(UpdateType::Enhancement)),
            "newpackage" => Ok(Some(UpdateType::NewPackage)),
            "bugfix" => Ok(Some(UpdateType::BugFix)),
            "security" => Ok(Some(UpdateType::Security)),
            _ => Err(format!("Not a recognised value for update type: {}", string)),
        },
        None => Ok(None),
    }
}

pub fn op_str_vec_to_op_release_vec(op_vec: Option<&Vec<String>>) -> Result<Option<Vec<FedoraRelease>>, String> {
    if let Some(strings) = op_vec {
        let mut result = Vec::new();

        for string in strings {
            match FedoraRelease::try_from(string.as_str()) {
                Ok(value) => result.push(value),
                Err(error) => return Err(error.to_string()),
            };
        };

        Ok(Some(result))
    } else {
        Ok(None)
    }
}

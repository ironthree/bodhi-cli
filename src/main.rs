use std::convert::TryFrom;
use std::fs::read_to_string;
use std::io::Write;

use bodhi::*;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Debug, Deserialize)]
struct FedoraConfig {
    #[serde(rename(deserialize = "FAS"))]
    fas: FASConfig,
}

#[derive(Debug, Deserialize)]
struct FASConfig {
    username: String,
}

enum Format {
    JSON,
    Plain,
}

impl TryFrom<&str> for Format {
    type Error = String;

    fn try_from(value: &str) -> Result<Format, String> {
        match value.to_lowercase().as_str() {
            "JSON" | "json" => Ok(Format::JSON),
            "plain" => Ok(Format::Plain),
            _ => return Err(format!("Not a recognised value for format: {}", &value)),
        }
    }
}

fn str_to_compose_request(request: &str) -> Result<ComposeRequest, String> {
    match request {
        "stable" => Ok(ComposeRequest::Stable),
        "testing" => Ok(ComposeRequest::Testing),
        _ => return Err(format!("Not a recognised value for compose request: {}", request)),
    }
}

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::DisableHelpSubcommand)]
struct BaseCommand {
    /// Use the fedora staging instance of bodhi
    #[structopt(long)]
    staging: bool,
    /// Manually specify bodhi server URL
    #[structopt(long, requires("login_url"), conflicts_with("staging"))]
    bodhi_url: Option<String>,
    /// Manually specify OpenID endpoint URL
    #[structopt(long, requires("bodhi_url"), conflicts_with("staging"))]
    login_url: Option<String>,
    #[structopt(subcommand)]
    subcommand: BodhiCommand,
}

#[derive(Debug, StructOpt)]
enum BodhiCommand {
    /// Comment on an update
    Comment {
        /// ID of the update to comment on
        #[structopt(long)]
        update: String,
        /// Publicly visible comment text
        #[structopt(long)]
        text: String,
        /// Karma submitted with this comment (-1/0/+1)
        #[structopt(long)]
        karma: Option<String>,
    },
    /// Query bodhi for information about a compose
    ComposeInfo {
        /// release string
        release: String,
        /// request string ("stable" or "testing")
        request: String,
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<String>,
    },
    /// Query bodhi for running composes
    ComposeList {
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<String>,
    },
    /// Create a new buildroot override
    CreateOverride {
        /// NVR of the override
        nvr: String,
        /// duration it will still be active
        #[structopt(long)]
        duration: u32,
        /// publicly visible notes
        #[structopt(long)]
        notes: String,
    },
    /// Create a new update
    CreateUpdate {
        /// Push to stable based on karma
        #[structopt(long)]
        autokarma: Option<bool>,
        /// Push to stable based on time
        #[structopt(long)]
        autotime: Option<bool>,
        /// Bugs fixed by this update
        #[structopt(long)]
        bugs: Option<Vec<u32>>,
        /// Builds that are part of this update
        #[structopt(long, conflicts_with = "from_tag")]
        builds: Option<Vec<String>>,
        /// Close bugs when pushed to stable
        #[structopt(long)]
        close_bugs: Option<bool>,
        /// Override displayed update name
        #[structopt(long)]
        display_name: Option<String>,
        /// Koji tag to create this update from
        #[structopt(long, conflicts_with = "builds")]
        from_tag: Option<String>,
        /// Publicly visible update notes
        #[structopt(long)]
        notes: String,
        /// List of required taskotron tests
        #[structopt(long)]
        requirements: Option<Vec<String>>,
        /// Update severity
        #[structopt(long)]
        severity: Option<String>,
        /// Days until it can be pushed to stable
        #[structopt(long)]
        stable_days: Option<u32>,
        /// Karma until it can be pushed to stable
        #[structopt(long)]
        stable_karma: Option<i32>,
        /// Logout / reboot suggestion
        #[structopt(long)]
        suggestion: Option<String>,
        /// Karma until it will be unpushed
        #[structopt(long)]
        unstable_karma: Option<i32>,
        /// Type of the update
        #[structopt(long, name = "type")]
        update_type: Option<String>,
    },
    /// Edit an existing buildroot override
    EditOverride {
        /// NVR of the override
        nvr: String,
        /// duration it will still be active
        #[structopt(long)]
        duration: u32,
        /// publicly visible notes
        #[structopt(long)]
        notes: String,
    },
    /// Edit an existing update
    EditUpdate {
        /// Alias of the edited update
        alias: String,
        /// Add bugs to this update
        #[structopt(long)]
        add_bugs: Option<Vec<u32>>,
        /// Add builds to this update
        #[structopt(long)]
        add_builds: Option<Vec<String>>,
        /// Push to stable based on karma
        #[structopt(long)]
        autokarma: Option<bool>,
        /// Push to stable based on time
        #[structopt(long)]
        autotime: Option<bool>,
        /// Close bugs when pushed to stable
        #[structopt(long)]
        close_bugs: Option<bool>,
        /// Override displayed update name
        #[structopt(long)]
        display_name: Option<String>,
        /// Publicly visible update notes
        #[structopt(long)]
        notes: Option<String>,
        /// Remove bugs from this update
        #[structopt(long)]
        remove_bugs: Option<Vec<u32>>,
        /// Remove builds from this update
        #[structopt(long)]
        remove_builds: Option<Vec<String>>,
        /// List of required taskotron tests
        #[structopt(long)]
        requirements: Option<Vec<String>>,
        /// Update severity
        #[structopt(long)]
        severity: Option<String>,
        /// Days until it can be pushed to stable
        #[structopt(long)]
        stable_days: Option<u32>,
        /// Karma until it can be pushed to stable
        #[structopt(long)]
        stable_karma: Option<i32>,
        /// Logout / reboot suggestion
        #[structopt(long)]
        suggestion: Option<String>,
        /// Karma until it will be unpushed
        #[structopt(long)]
        unstable_karma: Option<i32>,
        /// Type of the update
        #[structopt(long, name = "type")]
        update_type: Option<String>,
    },
    /// Expire an existing buildroot override
    ExpireOverride {
        /// NVR of the override
        nvr: String,
    },
    /// Query bodhi for buildroot overrides
    QueryOverrides {
        /// Query for this build / these builds
        #[structopt(long)]
        builds: Option<Vec<String>>,
        /// Query for expired overrides
        #[structopt(long)]
        expired: Option<bool>,
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<String>,
        /// Query for this release / these releases
        #[structopt(long)]
        releases: Option<Vec<String>>,
        /// Query for overrides submitted by these users
        #[structopt(long)]
        users: Option<Vec<String>>,
    },
    /// Query bodhi for updates
    QueryUpdates {
        /// update with this alias
        #[structopt(long)]
        alias: Option<String>,
        /// updates approved before this date
        #[structopt(long)]
        approved_before: Option<String>,
        /// updates approved after this date
        #[structopt(long)]
        approved_since: Option<String>,
        /// updates associated with these bugs
        #[structopt(long)]
        bugs: Option<Vec<u32>>,
        /// updates associated with these builds
        #[structopt(long)]
        builds: Option<Vec<String>>,
        /// updates for critpath packages
        #[structopt(long)]
        critpath: Option<bool>,
        /// RPM / module / flatpak updates
        #[structopt(long)]
        content_type: Option<String>,
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<String>,
        /// locked updates
        #[structopt(long)]
        locked: Option<bool>,
        /// updates modified before this date
        #[structopt(long)]
        modified_before: Option<String>,
        /// updates modified after this date
        #[structopt(long)]
        modified_since: Option<String>,
        /// updates for these packages
        #[structopt(long)]
        packages: Option<Vec<String>>,
        /// pushed updates
        #[structopt(long)]
        pushed: Option<bool>,
        /// updates pushed before this date
        #[structopt(long)]
        pushed_before: Option<String>,
        /// updates pushed after this date
        #[structopt(long)]
        pushed_since: Option<String>,
        /// updates for these releases
        #[structopt(long)]
        releases: Option<Vec<String>>,
        /// updates with this status request
        #[structopt(long)]
        request: Option<String>,
        /// updates with this severity
        #[structopt(long)]
        severity: Option<String>,
        /// updates with this status
        #[structopt(long)]
        status: Option<String>,
        /// updates submitted before this date
        #[structopt(long)]
        submitted_before: Option<String>,
        /// updates submitted after this date
        #[structopt(long)]
        submitted_since: Option<String>,
        /// updates with logout / reboot suggestion
        #[structopt(long)]
        suggestion: Option<String>,
        /// updates with this type
        #[structopt(name = "type", long)]
        update_type: Option<String>,
        /// updates submitted by this user
        #[structopt(long)]
        users: Option<Vec<String>>,
    },
    /// Query bodhi for information about a release
    ReleaseInfo {
        /// ID of the release
        release: String,
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<String>,
    },
    /// Query bodhi for active releases
    ReleaseList {
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<String>,
    },
    /// Submit an update status request
    UpdateRequest {
        /// ID of the update
        alias: String,
        /// (obsolete, revoke, stable, testing, unpush)
        request: String,
    },
    /// Waive an update's test results
    WaiveTests {
        /// ID of the update
        alias: String,
        /// comment submitted with the waiver
        comment: String,
    },
}

fn get_config() -> Result<FedoraConfig, String> {
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => {
            return Err(String::from("Unable to determine $HOME."));
        },
    };

    let config_path = home.join(".config/fedora.toml");

    let config_str = match read_to_string(&config_path) {
        Ok(string) => string,
        Err(_) => {
            return Err(String::from(
                "Unable to read configuration file from ~/.config/fedora.toml",
            ));
        },
    };

    let config: FedoraConfig = match toml::from_str(&config_str) {
        Ok(config) => config,
        Err(_) => {
            return Err(String::from(
                "Unable to parse configuration file from ~/.config/fedora.toml",
            ));
        },
    };

    Ok(config)
}

fn progress_bar(p: u32, ps: u32) {
    let columns: u32 = match term_size::dimensions() {
        Some((w, _)) => w as u32,
        None => return,
    };

    let width: u32 = columns - 11;

    let progress = ((p as f64) / (ps as f64) * (width as f64)) as u32;
    let remaining = width - progress;

    let bar = format!(
        " [ {}{} ] {:>3}% ",
        "=".repeat(progress as usize),
        " ".repeat(remaining as usize),
        ((p as f64) / (ps as f64) * 100f64) as u32,
    );

    print!("\r{}", &bar);
    std::io::stdout().flush().unwrap();
}

fn main() -> Result<(), String> {
    let args: BaseCommand = BaseCommand::from_args();

    let authenticated = match &args.subcommand {
        BodhiCommand::Comment { .. } => true,
        BodhiCommand::ComposeInfo { .. } => false,
        BodhiCommand::ComposeList { .. } => false,
        BodhiCommand::CreateOverride { .. } => true,
        BodhiCommand::CreateUpdate { .. } => true,
        BodhiCommand::EditOverride { .. } => true,
        BodhiCommand::EditUpdate { .. } => true,
        BodhiCommand::ExpireOverride { .. } => true,
        BodhiCommand::QueryOverrides { .. } => false,
        BodhiCommand::QueryUpdates { .. } => false,
        BodhiCommand::ReleaseInfo { .. } => false,
        BodhiCommand::ReleaseList { .. } => false,
        BodhiCommand::UpdateRequest { .. } => true,
        BodhiCommand::WaiveTests { .. } => true,
    };

    let config = get_config()?;

    let mut builder = match (&args.staging, &args.bodhi_url, &args.login_url) {
        (false, None, None) => BodhiServiceBuilder::default(),
        (true, None, None) => BodhiServiceBuilder::staging(),
        (false, Some(url), Some(login_url)) => BodhiServiceBuilder::custom(url.to_owned(), login_url.to_owned()),
        _ => unreachable!(),
    };

    let bodhi = if authenticated {
        let password =
            rpassword::prompt_password_stdout("FAS Password: ").expect("Failed to read password from command line.");

        builder = builder.authentication(&config.fas.username, &password);

        match builder.build() {
            Ok(bodhi) => bodhi,
            Err(error) => return Err(error.to_string()),
        }
    } else {
        match builder.build() {
            Ok(bodhi) => bodhi,
            Err(error) => return Err(error.to_string()),
        }
    };

    match args.subcommand {
        BodhiCommand::Comment { update, text, karma } => {
            let update: Update = match bodhi.query(UpdateIDQuery::new(&update)) {
                Ok(update) => match update {
                    Some(update) => update,
                    None => return Err(String::from("Update not found.")),
                },
                Err(error) => return Err(error.to_string()),
            };

            let karma = if let Some(karma) = &karma {
                match karma.as_str() {
                    "1" | "+1" => Some(Karma::Positive),
                    "0" => Some(Karma::Neutral),
                    "-1" => Some(Karma::Negative),
                    _ => return Err(format!("Not a recognised value for karma: {}", karma)),
                }
            } else {
                None
            };

            let mut commenter = update.comment().text(&text);

            if let Some(karma) = karma {
                commenter = commenter.karma(karma);
            }

            match bodhi.create(&commenter) {
                Ok(_) => {
                    println!("Comment created.");
                    Ok(())
                },
                Err(error) => Err(error.to_string()),
            }
        },
        BodhiCommand::ComposeInfo {
            release,
            request,
            format,
        } => {
            let release = FedoraRelease::try_from(release.as_str())?;
            let request = str_to_compose_request(request.as_str())?;

            let format = match format {
                Some(format) => Format::try_from(format.as_str())?,
                None => Format::Plain,
            };

            let result: Option<Compose> = match bodhi.query(ComposeReleaseRequestQuery::new(release, request)) {
                Ok(compose) => compose,
                Err(_) => return Err(format!("Failed to query composes.")),
            };

            match format {
                Format::Plain => match result {
                    Some(compose) => println!("{}", compose),
                    None => println!("No running compose found for: {}/{}", release, request),
                },
                Format::JSON => match result {
                    Some(compose) => {
                        let pretty = match serde_json::to_string_pretty(&compose) {
                            Ok(string) => string,
                            Err(_) => return Err(String::from("Failed to format output as JSON.")),
                        };

                        println!("{}", &pretty);
                    },
                    None => {},
                },
            }

            Ok(())
        },
        BodhiCommand::ComposeList { format } => {
            let format = match format {
                Some(format) => Format::try_from(format.as_str())?,
                None => Format::Plain,
            };

            let result: Vec<Compose> = match bodhi.query(ComposeQuery::new()) {
                Ok(composes) => composes,
                Err(_) => return Err(format!("Failed to query composes.")),
            };

            match format {
                Format::Plain => {
                    for compose in result {
                        println!("{}", compose);
                    }
                },
                Format::JSON => {
                    let pretty = match serde_json::to_string_pretty(&result) {
                        Ok(string) => string,
                        Err(_) => return Err(String::from("Failed to format output as JSON.")),
                    };

                    println!("{}", &pretty);
                },
            }

            Ok(())
        },
        BodhiCommand::CreateOverride { nvr, duration, notes } => {
            let current_date = chrono::Utc::now();
            let expiration_date = (current_date + chrono::Duration::days(duration as i64)).into();

            let builder = OverrideBuilder::new(&nvr, &notes, &expiration_date);

            let result: NewOverride = match bodhi.create(&builder) {
                Ok(o) => o,
                Err(error) => return Err(error.to_string()),
            };

            if !result.caveats.is_empty() {
                println!("Server messages:");
                for caveat in result.caveats {
                    for (key, value) in caveat {
                        println!("{}: {}", key, value);
                    }
                }
            };

            println!("{}", result.over_ride);

            Ok(())
        },
        BodhiCommand::CreateUpdate {
            autokarma,
            autotime,
            bugs,
            builds,
            close_bugs,
            display_name,
            from_tag,
            notes,
            requirements,
            severity,
            stable_days,
            stable_karma,
            suggestion,
            unstable_karma,
            update_type,
        } => {
            let builds: Option<Vec<&str>> = match &builds {
                Some(builds) => Some(builds.iter().map(|b| b.as_str()).collect()),
                None => None,
            };

            let mut builder = match (&builds, &from_tag) {
                (Some(_), Some(_)) => unreachable!(),
                (Some(builds), None) => UpdateBuilder::from_builds(&builds, &notes),
                (None, Some(tag)) => UpdateBuilder::from_tag(&tag, &notes),
                (None, None) => return Err(String::from("Neither builds nor koji tag specified.")),
            };

            if let Some(autokarma) = autokarma {
                builder = builder.autokarma(autokarma);
            };

            if let Some(autotime) = autotime {
                builder = builder.autotime(autotime);
            };

            if let Some(bugs) = bugs {
                for bug in bugs {
                    builder = builder.bugs(bug);
                }
            };

            if let Some(close_bugs) = close_bugs {
                builder = builder.close_bugs(close_bugs);
            };

            if let Some(display_name) = display_name {
                builder = builder.title(display_name);
            };

            if let Some(requirements) = &requirements {
                if !requirements.is_empty() {
                    builder = builder.requirements(requirements.join(","));
                };
            };

            if let Some(severity) = severity {
                let severity = match severity.to_lowercase().as_str() {
                    "unspecified" => UpdateSeverity::Unspecified,
                    "low" => UpdateSeverity::Low,
                    "medium" => UpdateSeverity::Medium,
                    "high" => UpdateSeverity::High,
                    "urgent" => UpdateSeverity::Urgent,
                    _ => return Err(format!("Not a recognised value for severity: {}", severity)),
                };

                builder = builder.severity(severity);
            };

            if let Some(stable_days) = stable_days {
                builder = builder.stable_days(stable_days);
            };

            if let Some(stable_karma) = stable_karma {
                builder = builder.stable_karma(stable_karma);
            };

            if let Some(suggestion) = suggestion {
                let suggestion = match suggestion.to_lowercase().as_str() {
                    "unspecified" => UpdateSuggestion::Unspecified,
                    "reboot" => UpdateSuggestion::Reboot,
                    "logout" => UpdateSuggestion::Logout,
                    _ => return Err(format!("Not a recognised value for suggestion: {}", suggestion)),
                };

                builder = builder.suggest(suggestion);
            };

            if let Some(unstable_karma) = unstable_karma {
                builder = builder.unstable_karma(unstable_karma);
            };

            if let Some(update_type) = update_type {
                let update_type = match update_type.to_lowercase().as_str() {
                    "unspecified" => UpdateType::Unspecified,
                    "enhancement" => UpdateType::Enhancement,
                    "newpackage" => UpdateType::NewPackage,
                    "bugfix" => UpdateType::BugFix,
                    "security" => UpdateType::Security,
                    _ => return Err(format!("Not a recognised value for update type: {}", update_type)),
                };

                builder = builder.update_type(update_type);
            };

            let result: NewUpdate = match bodhi.create(&builder) {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            if !result.caveats.is_empty() {
                println!("Server messages:");
                for caveat in result.caveats {
                    for (key, value) in caveat {
                        println!("{}: {}", key, value);
                    }
                }
            };

            println!("{}", result.update);

            Ok(())
        },
        BodhiCommand::EditOverride { nvr, duration, notes } => {
            let over_ride = match bodhi.query(OverrideNVRQuery::new(&nvr)) {
                Ok(value) => match value {
                    Some(over_ride) => over_ride,
                    None => return Err(format!("No override found for NVR: {}", nvr)),
                },
                Err(error) => return Err(error.to_string()),
            };

            let current_date = chrono::Utc::now();
            let expiration_date = (current_date + chrono::Duration::days(duration as i64)).into();

            let editor = OverrideEditor::from_override(&over_ride)
                .expiration_date(&expiration_date)
                .notes(&notes);

            let result: EditedOverride = match bodhi.edit(&editor) {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            if !result.caveats.is_empty() {
                println!("Server messages:");
                for caveat in result.caveats {
                    for (key, value) in caveat {
                        println!("{}: {}", key, value);
                    }
                }
            };

            println!("{}", result.over_ride);

            Ok(())
        },
        BodhiCommand::EditUpdate {
            alias,
            add_bugs,
            add_builds,
            autokarma,
            autotime,
            close_bugs,
            display_name,
            notes,
            remove_bugs,
            remove_builds,
            requirements,
            severity,
            stable_days,
            stable_karma,
            suggestion,
            unstable_karma,
            update_type,
        } => {
            let update = match bodhi.query(UpdateIDQuery::new(&alias)) {
                Ok(value) => match value {
                    Some(update) => update,
                    None => return Err(format!("No update found with this alias: {}", alias)),
                },
                Err(error) => return Err(error.to_string()),
            };

            let mut editor = UpdateEditor::from_update(&update);

            if let Some(add_bugs) = add_bugs {
                for bug in add_bugs {
                    editor = editor.add_bug(bug);
                }
            };

            if let Some(add_builds) = &add_builds {
                for build in add_builds {
                    editor = editor.add_build(build);
                }
            };

            if let Some(autokarma) = autokarma {
                editor = editor.autokarma(autokarma);
            };

            if let Some(autotime) = autotime {
                editor = editor.autotime(autotime);
            };

            if let Some(close_bugs) = close_bugs {
                editor = editor.close_bugs(close_bugs);
            };

            if let Some(display_name) = &display_name {
                editor = editor.set_title(display_name);
            };

            if let Some(notes) = &notes {
                editor = editor.notes(notes);
            }

            if let Some(remove_bugs) = remove_bugs {
                for bug in remove_bugs {
                    editor = editor.remove_bug(bug);
                }
            };

            if let Some(remove_builds) = &remove_builds {
                for build in remove_builds {
                    editor = editor.remove_build(build);
                }
            };

            let requirements = match &requirements {
                Some(reqs) => Some(reqs.join(",")),
                None => None,
            };
            if let Some(requirements) = &requirements {
                editor = editor.requirements(&requirements);
            }

            if let Some(severity) = severity {
                let severity = match severity.to_lowercase().as_str() {
                    "unspecified" => UpdateSeverity::Unspecified,
                    "low" => UpdateSeverity::Low,
                    "medium" => UpdateSeverity::Medium,
                    "high" => UpdateSeverity::High,
                    "urgent" => UpdateSeverity::Urgent,
                    _ => return Err(format!("Not a recognised value for severity: {}", severity)),
                };

                editor = editor.severity(severity);
            };


            if let Some(stable_days) = stable_days {
                editor = editor.stable_days(stable_days);
            };

            if let Some(stable_karma) = stable_karma {
                editor = editor.stable_karma(stable_karma);
            };

            if let Some(suggestion) = suggestion {
                let suggestion = match suggestion.to_lowercase().as_str() {
                    "unspecified" => UpdateSuggestion::Unspecified,
                    "reboot" => UpdateSuggestion::Reboot,
                    "logout" => UpdateSuggestion::Logout,
                    _ => return Err(format!("Not a recognised value for suggestion: {}", suggestion)),
                };

                editor = editor.suggest(suggestion);
            };

            if let Some(unstable_karma) = unstable_karma {
                editor = editor.unstable_karma(unstable_karma);
            };

            if let Some(update_type) = update_type {
                let update_type = match update_type.to_lowercase().as_str() {
                    "unspecified" => UpdateType::Unspecified,
                    "enhancement" => UpdateType::Enhancement,
                    "newpackage" => UpdateType::NewPackage,
                    "bugfix" => UpdateType::BugFix,
                    "security" => UpdateType::Security,
                    _ => return Err(format!("Not a recognised value for update type: {}", update_type)),
                };

                editor = editor.update_type(update_type);
            }

            let result: EditedUpdate = match bodhi.edit(&editor) {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            if !result.caveats.is_empty() {
                println!("Server messages:");
                for caveat in result.caveats {
                    for (key, value) in caveat {
                        println!("{}: {}", key, value);
                    }
                }
            };

            println!("{}", result.update);

            Ok(())
        },
        BodhiCommand::ExpireOverride { nvr } => {
            let over_ride = match bodhi.query(OverrideNVRQuery::new(&nvr)) {
                Ok(value) => match value {
                    Some(over_ride) => over_ride,
                    None => return Err(format!("No override found for NVR: {}", nvr)),
                },
                Err(error) => return Err(error.to_string()),
            };

            let editor = OverrideEditor::from_override(&over_ride).expired(true);

            let result: EditedOverride = match bodhi.edit(&editor) {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            if !result.caveats.is_empty() {
                println!("Server messages:");
                for caveat in result.caveats {
                    for (key, value) in caveat {
                        println!("{}: {}", key, value);
                    }
                }
            };

            println!("{}", result.over_ride);

            Ok(())
        },
        BodhiCommand::QueryOverrides {
            builds,
            expired,
            format,
            releases,
            users,
        } => {
            let format = match format {
                Some(format) => Format::try_from(format.as_str())?,
                None => Format::Plain,
            };

            let mut query = OverrideQuery::new();

            if let Some(builds) = &builds {
                for build in builds {
                    query = query.builds(build);
                }
            };

            if let Some(expired) = expired {
                query = query.expired(expired);
            };

            if let Some(releases) = releases {
                for release in releases {
                    let release = match FedoraRelease::try_from(release.as_str()) {
                        Ok(value) => value,
                        Err(error) => return Err(error.to_string()),
                    };

                    query = query.releases(release);
                }
            };

            if let Some(users) = &users {
                for user in users {
                    query = query.users(user);
                }
            };

            if let Format::Plain = format {
                query = query.callback(progress_bar)
            };

            let result: Vec<Override> = match bodhi.query(query) {
                Ok(overrides) => overrides,
                Err(_) => return Err(format!("Failed to query overrides.")),
            };

            match format {
                Format::Plain => {
                    for over_ride in result {
                        println!("{}", over_ride);
                    }
                },
                Format::JSON => {
                    let pretty = match serde_json::to_string_pretty(&result) {
                        Ok(string) => string,
                        Err(_) => return Err(String::from("Failed to format output as JSON.")),
                    };

                    println!("{}", &pretty);
                },
            }

            Ok(())
        },
        BodhiCommand::QueryUpdates {
            alias,
            approved_before,
            approved_since,
            bugs,
            builds,
            critpath,
            content_type,
            format,
            locked,
            modified_before,
            modified_since,
            packages,
            pushed,
            pushed_before,
            pushed_since,
            releases,
            request,
            severity,
            status,
            submitted_before,
            submitted_since,
            suggestion,
            update_type,
            users,
        } => {
            let format = match format {
                Some(format) => Format::try_from(format.as_str())?,
                None => Format::Plain,
            };

            let mut query = UpdateQuery::new();

            if let Some(alias) = &alias {
                query = query.aliases(alias);
            };

            let approved_before = match approved_before {
                Some(date) => match BodhiDate::try_from(date.as_str()) {
                    Ok(value) => Some(value),
                    Err(_) => return Err(format!("Date in invalid format: {}", date)),
                },
                None => None,
            };
            if let Some(approved_before) = approved_before {
                query = query.approved_before(approved_before);
            };

            let approved_since = match approved_since {
                Some(date) => match BodhiDate::try_from(date.as_str()) {
                    Ok(value) => Some(value),
                    Err(_) => return Err(format!("Date in invalid format: {}", date)),
                },
                None => None,
            };
            if let Some(approved_since) = approved_since {
                query = query.approved_since(approved_since);
            };

            if let Some(bugs) = bugs {
                for bug in bugs {
                    query = query.bugs(bug);
                }
            };

            if let Some(builds) = &builds {
                for build in builds {
                    query = query.builds(build);
                }
            };

            if let Some(critpath) = critpath {
                query = query.critpath(critpath);
            };

            if let Some(content_type) = content_type {
                let content_type = match content_type.to_lowercase().as_str() {
                    "rpm" => ContentType::RPM,
                    "module" => ContentType::Module,
                    "flatpak" => ContentType::Flatpak,
                    "container" => ContentType::Container,
                    _ => return Err(format!("Not a recognised content type: {}", content_type)),
                };

                query = query.content_type(content_type);
            };

            if let Some(locked) = locked {
                query = query.locked(locked);
            };

            let modified_before = match modified_before {
                Some(date) => match BodhiDate::try_from(date.as_str()) {
                    Ok(value) => Some(value),
                    Err(_) => return Err(format!("Date in invalid format: {}", date)),
                },
                None => None,
            };
            if let Some(modified_before) = modified_before {
                query = query.modified_before(modified_before);
            };

            let modified_since = match modified_since {
                Some(date) => match BodhiDate::try_from(date.as_str()) {
                    Ok(value) => Some(value),
                    Err(_) => return Err(format!("Date in invalid format: {}", date)),
                },
                None => None,
            };
            if let Some(modified_since) = modified_since {
                query = query.modified_since(modified_since);
            };

            if let Some(packages) = &packages {
                for package in packages {
                    query = query.packages(package);
                }
            };

            if let Some(pushed) = pushed {
                query = query.pushed(pushed);
            };

            let pushed_before = match pushed_before {
                Some(date) => match BodhiDate::try_from(date.as_str()) {
                    Ok(value) => Some(value),
                    Err(_) => return Err(format!("Date in invalid format: {}", date)),
                },
                None => None,
            };
            if let Some(pushed_before) = pushed_before {
                query = query.pushed_before(pushed_before);
            };

            let pushed_since = match pushed_since {
                Some(date) => match BodhiDate::try_from(date.as_str()) {
                    Ok(value) => Some(value),
                    Err(_) => return Err(format!("Date in invalid format: {}", date)),
                },
                None => None,
            };
            if let Some(pushed_since) = pushed_since {
                query = query.pushed_since(pushed_since);
            };

            if let Some(releases) = releases {
                for release in releases {
                    let release = match FedoraRelease::try_from(release.as_str()) {
                        Ok(value) => value,
                        Err(error) => return Err(error.to_string()),
                    };

                    query = query.releases(release);
                }
            };

            if let Some(request) = request {
                let request = match request.to_lowercase().as_str() {
                    "obsolete" => UpdateRequest::Obsolete,
                    "revoke" => UpdateRequest::Revoke,
                    "stable" => UpdateRequest::Stable,
                    "testing" => UpdateRequest::Testing,
                    "unpush" => UpdateRequest::Unpush,
                    _ => return Err(format!("Not a recognised value for update request: {}", request)),
                };

                query = query.request(request);
            };

            if let Some(severity) = severity {
                let severity = match severity.to_lowercase().as_str() {
                    "unspecified" => UpdateSeverity::Unspecified,
                    "low" => UpdateSeverity::Low,
                    "medium" => UpdateSeverity::Medium,
                    "high" => UpdateSeverity::High,
                    "urgent" => UpdateSeverity::Urgent,
                    _ => return Err(format!("Not a recognised value for severity: {}", severity)),
                };

                query = query.severity(severity);
            };

            if let Some(status) = status {
                let status = match status.to_lowercase().as_str() {
                    "obsolete" => UpdateStatus::Obsolete,
                    "pending" => UpdateStatus::Pending,
                    "side_tag_active" => UpdateStatus::SideTagActive,
                    "side_tag_expired" => UpdateStatus::SideTagExpired,
                    "stable" => UpdateStatus::Stable,
                    "testing" => UpdateStatus::Testing,
                    "unpushed" => UpdateStatus::Unpushed,
                    _ => return Err(format!("Not a recognised value for status: {}", status)),
                };

                query = query.status(status);
            };

            let submitted_before = match submitted_before {
                Some(date) => match BodhiDate::try_from(date.as_str()) {
                    Ok(value) => Some(value),
                    Err(_) => return Err(format!("Date in invalid format: {}", date)),
                },
                None => None,
            };
            if let Some(submitted_before) = submitted_before {
                query = query.submitted_before(submitted_before);
            };

            let submitted_since = match submitted_since {
                Some(date) => match BodhiDate::try_from(date.as_str()) {
                    Ok(value) => Some(value),
                    Err(_) => return Err(format!("Date in invalid format: {}", date)),
                },
                None => None,
            };
            if let Some(submitted_since) = submitted_since {
                query = query.submitted_since(submitted_since);
            };

            if let Some(suggestion) = suggestion {
                let suggestion = match suggestion.to_lowercase().as_str() {
                    "unspecified" => UpdateSuggestion::Unspecified,
                    "reboot" => UpdateSuggestion::Reboot,
                    "logout" => UpdateSuggestion::Logout,
                    _ => return Err(format!("Not a recognised value for suggestion: {}", suggestion)),
                };

                query = query.suggest(suggestion);
            }

            if let Some(update_type) = update_type {
                let update_type = match update_type.to_lowercase().as_str() {
                    "unspecified" => UpdateType::Unspecified,
                    "enhancement" => UpdateType::Enhancement,
                    "newpackage" => UpdateType::NewPackage,
                    "bugfix" => UpdateType::BugFix,
                    "security" => UpdateType::Security,
                    _ => return Err(format!("Not a recognised value for update type: {}", update_type)),
                };

                query = query.update_type(update_type);
            };

            if let Some(users) = &users {
                for user in users {
                    query = query.users(user);
                }
            };

            if let Format::Plain = format {
                query = query.callback(progress_bar)
            };

            let result: Vec<Update> = match bodhi.query(query) {
                Ok(updates) => updates,
                Err(_) => return Err(format!("Failed to query updates.")),
            };

            match format {
                Format::Plain => {
                    for update in result {
                        println!("{}", update);
                    }
                },
                Format::JSON => {
                    let pretty = match serde_json::to_string_pretty(&result) {
                        Ok(string) => string,
                        Err(_) => return Err(String::from("Failed to format output as JSON.")),
                    };

                    println!("{}", &pretty);
                },
            }

            Ok(())
        },
        BodhiCommand::ReleaseInfo { release, format } => {
            FedoraRelease::try_from(release.as_str())?;

            let format = match format {
                Some(format) => Format::try_from(format.as_str())?,
                None => Format::Plain,
            };

            let result: Option<Release> = match bodhi.query(ReleaseNameQuery::new(&release)) {
                Ok(compose) => compose,
                Err(_) => return Err(String::from("Failed to query releases.")),
            };

            match format {
                Format::Plain => match result {
                    Some(release) => println!("{}", release),
                    None => println!("No release found with name: {}", &release),
                },
                Format::JSON => match result {
                    Some(release) => {
                        let pretty = match serde_json::to_string_pretty(&release) {
                            Ok(string) => string,
                            Err(_) => return Err(String::from("Failed to format output as JSON.")),
                        };

                        println!("{}", &pretty);
                    },
                    None => {},
                },
            }

            Ok(())
        },
        BodhiCommand::ReleaseList { format } => {
            let format = match format {
                Some(format) => Format::try_from(format.as_str())?,
                None => Format::Plain,
            };

            let result: Vec<Release> = match bodhi.query(ReleaseQuery::new()) {
                Ok(releases) => releases,
                Err(_) => return Err(format!("Failed to query releases.")),
            };

            match format {
                Format::Plain => {
                    for release in result {
                        println!("{}", release);
                    }
                },
                Format::JSON => {
                    let pretty = match serde_json::to_string_pretty(&result) {
                        Ok(string) => string,
                        Err(_) => return Err(String::from("Failed to format output as JSON.")),
                    };

                    println!("{}", &pretty);
                },
            }

            Ok(())
        },
        BodhiCommand::UpdateRequest { alias, request } => {
            let request = match request.to_lowercase().as_str() {
                "obsolete" => UpdateRequest::Obsolete,
                "revoke" => UpdateRequest::Revoke,
                "stable" => UpdateRequest::Stable,
                "testing" => UpdateRequest::Testing,
                "unpush" => UpdateRequest::Unpush,
                _ => return Err(format!("Not a recognised value for update request: {}", request)),
            };

            let update: Update = match bodhi.query(UpdateIDQuery::new(&alias)) {
                Ok(value) => match value {
                    Some(update) => update,
                    None => return Err(format!("No update found with this alias: {}", alias)),
                },
                Err(error) => return Err(error.to_string()),
            };

            let editor = UpdateStatusRequester::from_update(&update, request);

            let result: Update = match bodhi.edit(&editor) {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            println!("{}", result);

            Ok(())
        },
        BodhiCommand::WaiveTests { alias, comment } => {
            let update: Update = match bodhi.query(UpdateIDQuery::new(&alias)) {
                Ok(value) => match value {
                    Some(update) => update,
                    None => return Err(format!("No update found with this alias: {}", alias)),
                },
                Err(error) => return Err(error.to_string()),
            };

            let editor = UpdateTestResultWaiver::from_update(&update, &comment);

            let result: Update = match bodhi.edit(&editor) {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            println!("{}", result);

            Ok(())
        },
    }
}

#![warn(clippy::unwrap_used)]

use std::collections::HashMap;

use bodhi::*;
use structopt::StructOpt;

pub mod cli;
pub use cli::*;

pub mod config;
pub use config::*;

pub mod output;
pub use output::*;

pub mod query;
pub use query::*;

/// This function prompts the user for their FAS password.
fn read_password() -> String {
    rpassword::prompt_password_stdout("FAS Password: ").expect("Failed to read from console.")
}

/// This function asks for and stores the password in the session keyring.
fn get_store_password(clear: bool) -> Result<String, String> {
    let ss = match secret_service::SecretService::new(secret_service::EncryptionType::Dh) {
        Ok(ss) => ss,
        Err(error) => {
            println!("Failed to initialize SecretService client: {}", error);
            return Ok(read_password());
        },
    };

    let collection = match ss.get_default_collection() {
        Ok(c) => c,
        Err(error) => {
            println!("Failed to query SecretService: {}", error);
            return Ok(read_password());
        },
    };

    let mut attributes = HashMap::new();
    attributes.insert("bodhi-cli", "FAS Password");

    let store = |password: &str, replace: bool| {
        if let Err(error) = collection.create_item(
            "bodhi-cli",
            attributes.clone(),
            password.as_bytes(),
            replace,
            "password",
        ) {
            println!("Failed to save password with SecretService: {}", error);
        }
    };

    let items = match collection.search_items(attributes.clone()) {
        Ok(items) => items,
        Err(error) => {
            format!("Failed to query SecretService: {}", error);
            return Ok(read_password());
        },
    };

    if clear {
        let password = read_password();
        store(&password, true);
        return Ok(password);
    };

    let password = match items.get(0) {
        Some(item) => match item.get_secret() {
            Ok(secret) => match String::from_utf8(secret) {
                Ok(valid) => valid,
                Err(error) => {
                    println!("Stored password was not valid UTF-8: {}", error);

                    let password = read_password();
                    store(&password, true);

                    password
                },
            },
            Err(error) => {
                println!("Password was not stored correctly: {}", error);

                let password = read_password();
                store(&password, true);

                password
            },
        },
        None => {
            let password = read_password();
            store(&password, false);

            password
        },
    };

    Ok(password)
}

#[allow(clippy::cognitive_complexity)]
#[tokio::main]
async fn main() -> Result<(), String> {
    let args: BaseCommand = BaseCommand::from_args();
    let authenticated = args.authenticated();

    let config = get_config().await?;

    let mut builder = match (&args.staging, &args.bodhi_url, &args.login_url) {
        (false, None, None) => BodhiClientBuilder::default(),
        (true, None, None) => BodhiClientBuilder::staging(),
        (false, Some(url), Some(login_url)) => BodhiClientBuilder::custom(url.to_owned(), login_url.to_owned()),
        _ => unreachable!(),
    };

    let bodhi = if authenticated {
        println!("Authenticating with bodhi ...");
        println!("Username: {}", &config.fas.username);

        let password = if !args.no_store_password {
            get_store_password(args.ignore_keyring)?
        } else {
            read_password()
        };

        builder = builder.authentication(&config.fas.username, &password);

        match builder.build().await {
            Ok(bodhi) => bodhi,
            Err(error) => return Err(error.to_string()),
        }
    } else {
        match builder.build().await {
            Ok(bodhi) => bodhi,
            Err(error) => return Err(error.to_string()),
        }
    };

    match args.subcommand {
        BodhiCommand::Comment { update, text, karma } => {
            let update: Update = match bodhi.request(&UpdateIDQuery::new(&update)).await {
                Ok(update) => update,
                Err(error) => return Err(error.to_string()),
            };

            let mut commenter = update.comment().text(&text);

            if let Some(karma) = karma {
                commenter = commenter.karma(karma);
            }

            match bodhi.request(&commenter).await {
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
            let result: Compose = match bodhi.request(&ComposeReleaseRequestQuery::new(&release, request)).await {
                Ok(compose) => compose,
                Err(error) => return Err(error.to_string()),
            };

            pretty_output(&result, format.unwrap_or(Format::Plain))?;

            Ok(())
        },
        BodhiCommand::ComposeList { format } => {
            let result: Vec<Compose> = match bodhi.request(&ComposeQuery::new()).await {
                Ok(composes) => composes,
                Err(error) => return Err(error.to_string()),
            };

            pretty_outputs(&result, format.unwrap_or(Format::Plain))?;

            Ok(())
        },
        BodhiCommand::CreateOverride { nvr, duration, notes } => {
            let current_date = chrono::Utc::now();
            let expiration_date = (current_date + chrono::Duration::days(duration as i64)).into();

            let builder = OverrideCreator::new(&nvr, &notes, &expiration_date);

            let result: NewOverride = match bodhi.request(&builder).await {
                Ok(o) => o,
                Err(error) => return Err(error.to_string()),
            };

            print_server_msgs(&result.caveats);
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
            require_bugs,
            require_testcases,
            requirements,
            severity,
            stable_days,
            stable_karma,
            suggestion,
            unstable_karma,
            update_type,
        } => {
            let builds: Option<Vec<&str>> = builds
                .as_ref()
                .map(|builds| builds.iter().map(|b| b.as_str()).collect());

            let mut builder = match (&builds, &from_tag) {
                (Some(_), Some(_)) => unreachable!(),
                (Some(builds), None) => UpdateCreator::from_builds(builds, &notes),
                (None, Some(tag)) => UpdateCreator::from_tag(tag, &notes),
                (None, None) => return Err(String::from("Neither builds nor koji tag specified.")),
            };

            if let Some(autokarma) = autokarma {
                builder = builder.autokarma(autokarma);
            };

            if let Some(autotime) = autotime {
                builder = builder.autotime(autotime);
            };

            if let Some(bugs) = &bugs {
                builder = builder.bugs(bugs);
            };

            if let Some(close_bugs) = close_bugs {
                builder = builder.close_bugs(close_bugs);
            };

            if let Some(display_name) = &display_name {
                builder = builder.display_name(display_name);
            };

            if let Some(require_bugs) = require_bugs {
                builder = builder.require_bugs(require_bugs);
            };

            if let Some(require_testcases) = require_testcases {
                builder = builder.require_testcases(require_testcases);
            };

            let requirements_string = requirements.map(|some| some.join(","));
            if let Some(requirements) = &requirements_string {
                if !requirements.is_empty() {
                    builder = builder.requirements(requirements);
                };
            };

            if let Some(severity) = severity {
                builder = builder.severity(severity);
            };

            if let Some(stable_days) = stable_days {
                builder = builder.stable_days(stable_days);
            };

            if let Some(stable_karma) = stable_karma {
                builder = builder.stable_karma(stable_karma);
            };

            if let Some(suggestion) = suggestion {
                builder = builder.suggest(suggestion);
            };

            if let Some(unstable_karma) = unstable_karma {
                builder = builder.unstable_karma(unstable_karma);
            };

            if let Some(update_type) = update_type {
                builder = builder.update_type(update_type);
            };

            let result: NewUpdate = match bodhi.request(&builder).await {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            print_server_msgs(&result.caveats);
            println!("{}", result.update);

            Ok(())
        },
        BodhiCommand::EditOverride { nvr, duration, notes } => {
            let current_date = chrono::Utc::now();
            let expiration_date = (current_date + chrono::Duration::days(duration as i64)).into();

            let over_ride = query_override(&bodhi, &nvr).await?;
            let editor = OverrideEditor::from_override(&over_ride)
                .expiration_date(&expiration_date)
                .notes(&notes);

            let result: EditedOverride = match bodhi.request(&editor).await {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            print_server_msgs(&result.caveats);
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
            let update = query_update(&bodhi, &alias).await?;
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
                editor = editor.display_name(display_name);
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

            let requirements = requirements.as_ref().map(|reqs| reqs.join(","));
            if let Some(requirements) = &requirements {
                editor = editor.requirements(requirements);
            }

            if let Some(severity) = severity {
                editor = editor.severity(severity);
            };

            if let Some(stable_days) = stable_days {
                editor = editor.stable_days(stable_days);
            };

            if let Some(stable_karma) = stable_karma {
                editor = editor.stable_karma(stable_karma);
            };

            if let Some(suggestion) = suggestion {
                editor = editor.suggest(suggestion);
            };

            if let Some(unstable_karma) = unstable_karma {
                editor = editor.unstable_karma(unstable_karma);
            };

            if let Some(update_type) = update_type {
                editor = editor.update_type(update_type);
            }

            let result: EditedUpdate = match bodhi.request(&editor).await {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            print_server_msgs(&result.caveats);
            println!("{}", result.update);

            Ok(())
        },
        BodhiCommand::ExpireOverride { nvr } => {
            let over_ride = query_override(&bodhi, &nvr).await?;
            let editor = OverrideEditor::from_override(&over_ride).expired(true);

            let result: EditedOverride = match bodhi.request(&editor).await {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            print_server_msgs(&result.caveats);
            println!("{}", result.over_ride);

            Ok(())
        },
        BodhiCommand::QueryOverrides {
            builds,
            expired,
            format,
            releases,
            users,
            force,
        } => {
            let format = format.unwrap_or(Format::Plain);

            let build_refs: Option<Vec<&str>> = builds.as_ref().map(|bs| bs.iter().map(|b| b.as_str()).collect());
            let user_refs: Option<Vec<&str>> = users.as_ref().map(|us| us.iter().map(|u| u.as_str()).collect());

            let mut query = OverrideQuery::new();
            let mut long_running = true;

            if let Some(build_refs) = &build_refs {
                query = query.builds(build_refs);
                long_running = false;
            };

            if let Some(expired) = expired {
                query = query.expired(expired);
                long_running = false;
            };

            if let Some(releases) = &releases {
                query = query.releases(releases);
                long_running = false;
            };

            if let Some(user_refs) = &user_refs {
                query = query.users(user_refs);
                long_running = false;
            };

            if let Format::Plain = format {
                query = query.callback(progress_bar)
            };

            if long_running && !force {
                eprintln!("Querying overrides without filters takes a *long* time. This is probably not");
                eprintln!("what you want to do. To do it anyway, use the '--force' flag.");

                return Ok(());
            }

            let result: Vec<Override> = match bodhi.paginated_request(&query).await {
                Ok(overrides) => overrides,
                Err(error) => return Err(error.to_string()),
            };

            pretty_outputs(&result, format)?;

            Ok(())
        },
        BodhiCommand::QueryUpdates {
            alias,
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
            force,
        } => {
            let format = format.unwrap_or(Format::Plain);

            let aliases = alias.as_ref().map(|alias| vec![alias.as_str()]);
            let build_refs: Option<Vec<&str>> = builds.as_ref().map(|bs| bs.iter().map(|b| b.as_str()).collect());
            let pkg_refs: Option<Vec<&str>> = packages.as_ref().map(|ps| ps.iter().map(|p| p.as_str()).collect());
            let user_refs: Option<Vec<&str>> = users.as_ref().map(|us| us.iter().map(|u| u.as_str()).collect());

            let mut query = UpdateQuery::new();
            let mut long_running = true;

            if let Some(aliases) = &aliases {
                query = query.aliases(aliases);
                long_running = false;
            };

            if let Some(bugs) = &bugs {
                query = query.bugs(bugs);
                long_running = false;
            };

            if let Some(build_refs) = &build_refs {
                query = query.builds(build_refs);
                long_running = false;
            };

            if let Some(critpath) = critpath {
                query = query.critpath(critpath);
                long_running = false;
            };

            if let Some(content_type) = content_type {
                query = query.content_type(content_type);
                long_running = false;
            };

            if let Some(locked) = locked {
                query = query.locked(locked);
                long_running = false;
            };

            if let Some(modified_before) = &modified_before {
                query = query.modified_before(modified_before);
                long_running = false;
            };

            if let Some(modified_since) = &modified_since {
                query = query.modified_since(modified_since);
                long_running = false;
            };

            if let Some(pkg_refs) = &pkg_refs {
                query = query.packages(pkg_refs);
                long_running = false;
            };

            if let Some(pushed) = pushed {
                query = query.pushed(pushed);
                long_running = false;
            };

            if let Some(pushed_before) = &pushed_before {
                query = query.pushed_before(pushed_before);
                long_running = false;
            };

            if let Some(pushed_since) = &pushed_since {
                query = query.pushed_since(pushed_since);
                long_running = false;
            };

            if let Some(releases) = &releases {
                query = query.releases(releases);
                long_running = false;
            };

            if let Some(request) = request {
                query = query.request(request);
                long_running = false;
            };

            if let Some(severity) = severity {
                query = query.severity(severity);
                long_running = false;
            };

            if let Some(status) = status {
                query = query.status(status);
                long_running = false;
            };

            if let Some(submitted_before) = &submitted_before {
                query = query.submitted_before(submitted_before);
                long_running = false;
            };

            if let Some(submitted_since) = &submitted_since {
                query = query.submitted_since(submitted_since);
                long_running = false;
            };

            if let Some(suggestion) = suggestion {
                query = query.suggest(suggestion);
                long_running = false;
            }

            if let Some(update_type) = update_type {
                query = query.update_type(update_type);
                long_running = false;
            };

            if let Some(user_refs) = &user_refs {
                query = query.users(user_refs);
                long_running = false;
            };

            if let Format::Plain = format {
                query = query.callback(progress_bar)
            };

            if long_running && !force {
                eprintln!("Querying updates without filters takes a *long* time. This is probably not");
                eprintln!("what you want to do. To do it anyway, use the '--force' flag.");

                return Ok(());
            }

            let result: Vec<Update> = match bodhi.paginated_request(&query).await {
                Ok(updates) => updates,
                Err(error) => return Err(error.to_string()),
            };

            pretty_outputs(&result, format)?;

            Ok(())
        },
        BodhiCommand::ReleaseInfo { release, format } => {
            let result: Release = match bodhi.request(&ReleaseNameQuery::new(&release)).await {
                Ok(release) => release,
                Err(_) => return Err(String::from("Failed to query releases.")),
            };

            pretty_output(&result, format.unwrap_or(Format::Plain))?;

            Ok(())
        },
        BodhiCommand::ReleaseList { format } => {
            let result: Vec<Release> = match bodhi.paginated_request(&ReleaseQuery::new()).await {
                Ok(releases) => releases,
                Err(error) => return Err(error.to_string()),
            };

            pretty_outputs(&result, format.unwrap_or(Format::Plain))?;

            Ok(())
        },
        BodhiCommand::UpdateRequest { alias, request } => {
            let update: Update = query_update(&bodhi, &alias).await?;
            let editor = UpdateStatusRequester::from_update(&update, request);

            let result: Update = match bodhi.request(&editor).await {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            println!("{}", result);

            Ok(())
        },
        BodhiCommand::WaiveTests { alias, comment, tests } => {
            let update = query_update(&bodhi, &alias).await?;

            let test_refs: Option<Vec<&str>> = tests.as_ref().map(|ts| ts.iter().map(|t| t.as_str()).collect());

            let mut editor = UpdateTestResultWaiver::from_update(&update, &comment);

            if let Some(test_refs) = &test_refs {
                editor = editor.tests(test_refs)
            }

            let result: Update = match bodhi.request(&editor).await {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            println!("{}", result);

            Ok(())
        },
    }
}

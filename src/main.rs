use bodhi::*;
use structopt::StructOpt;

use bodhi_cli::*;

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
            let result: Option<Compose> = match bodhi.query(ComposeReleaseRequestQuery::new(release, request)) {
                Ok(compose) => compose,
                Err(error) => return Err(error.to_string()),
            };

            pretty_output(result.as_ref(), &format!("{}/{}", release, request), "No running compose found for", format.unwrap_or(Format::Plain))?;

            Ok(())
        },
        BodhiCommand::ComposeList { format } => {
            let result: Vec<Compose> = match bodhi.query(ComposeQuery::new()) {
                Ok(composes) => composes,
                Err(error) => return Err(error.to_string()),
            };

            pretty_outputs(&result, format.unwrap_or(Format::Plain))?;

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

            let result: NewUpdate = match bodhi.create(&builder) {
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

            let over_ride = query_override(&bodhi, &nvr)?;
            let editor = OverrideEditor::from_override(&over_ride)
                .expiration_date(&expiration_date)
                .notes(&notes);

            let result: EditedOverride = match bodhi.edit(&editor) {
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
            let update = query_update(&bodhi, &alias)?;
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

            let result: EditedUpdate = match bodhi.edit(&editor) {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            print_server_msgs(&result.caveats);
            println!("{}", result.update);

            Ok(())
        },
        BodhiCommand::ExpireOverride { nvr } => {
            let over_ride = query_override(&bodhi, &nvr)?;
            let editor = OverrideEditor::from_override(&over_ride).expired(true);

            let result: EditedOverride = match bodhi.edit(&editor) {
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
        } => {
            let format = format.unwrap_or(Format::Plain);

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
                Err(error) => return Err(error.to_string()),
            };

            pretty_outputs(&result, format)?;

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
            let format = format.unwrap_or(Format::Plain);

            let mut query = UpdateQuery::new();

            if let Some(alias) = &alias {
                query = query.aliases(alias);
            };

            if let Some(approved_before) = &approved_before {
                query = query.approved_before(approved_before);
            };

            if let Some(approved_since) = &approved_since {
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
                query = query.content_type(content_type);
            };

            if let Some(locked) = locked {
                query = query.locked(locked);
            };

            if let Some(modified_before) = &modified_before {
                query = query.modified_before(modified_before);
            };

            if let Some(modified_since) = &modified_since {
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


            if let Some(pushed_before) = &pushed_before {
                query = query.pushed_before(pushed_before);
            };


            if let Some(pushed_since) = &pushed_since {
                query = query.pushed_since(pushed_since);
            };

            if let Some(releases) = releases {
                for release in releases {
                    query = query.releases(release);
                }
            };

            if let Some(request) = request {
                query = query.request(request);
            };

            if let Some(severity) = severity {
                query = query.severity(severity);
            };

            if let Some(status) = status {
                query = query.status(status);
            };

            if let Some(submitted_before) = &submitted_before {
                query = query.submitted_before(submitted_before);
            };

            if let Some(submitted_since) = &submitted_since {
                query = query.submitted_since(submitted_since);
            };

            if let Some(suggestion) = suggestion {
                query = query.suggest(suggestion);
            }

            if let Some(update_type) = update_type {
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
                Err(error) => return Err(error.to_string()),
            };

            pretty_outputs(&result, format)?;

            Ok(())
        },
        BodhiCommand::ReleaseInfo { release, format } => {
            let result: Option<Release> = match bodhi.query(ReleaseNameQuery::new(&release)) {
                Ok(compose) => compose,
                Err(_) => return Err(String::from("Failed to query releases.")),
            };

            pretty_output(result.as_ref(), &release, "No release found with name", format.unwrap_or(Format::Plain))?;

            Ok(())
        },
        BodhiCommand::ReleaseList { format } => {
            let result: Vec<Release> = match bodhi.query(ReleaseQuery::new()) {
                Ok(releases) => releases,
                Err(error) => return Err(error.to_string()),
            };

            pretty_outputs(&result, format.unwrap_or(Format::Plain))?;

            Ok(())
        },
        BodhiCommand::UpdateRequest { alias, request } => {
            let update: Update = query_update(&bodhi, &alias)?;
            let editor = UpdateStatusRequester::from_update(&update, request);

            let result: Update = match bodhi.edit(&editor) {
                Ok(value) => value,
                Err(error) => return Err(error.to_string()),
            };

            println!("{}", result);

            Ok(())
        },
        BodhiCommand::WaiveTests { alias, comment } => {
            let update = query_update(&bodhi, &alias)?;
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

use bodhi::*;
use structopt::StructOpt;

use crate::Format;

/// bodhi-cli expects a configuration file at ~/.config/fedora.toml, with at
/// least the following contents:
///
/// """
/// [FAS]
/// username = "USERNAME"
/// """
///
/// This username is used for logging in with bodhi for authenticated requests,
/// and for determining which updates, overrides, and comments the user has
/// created themselves.
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::DisableHelpSubcommand)]
#[structopt(verbatim_doc_comment)]
pub struct BaseCommand {
    /// Use the fedora staging instance of bodhi
    #[structopt(long)]
    pub staging: bool,
    /// Manually specify bodhi server URL
    #[structopt(long, requires("login_url"), conflicts_with("staging"))]
    pub bodhi_url: Option<String>,
    /// Manually specify OpenID endpoint URL
    #[structopt(long, requires("bodhi_url"), conflicts_with("staging"))]
    pub login_url: Option<String>,
    /// Don't store password in session keyring
    #[structopt(long, short = "n")]
    pub no_store_password: bool,
    /// Ignore password stored in session keyring
    #[structopt(long, short = "k")]
    pub ignore_keyring: bool,
    #[structopt(subcommand)]
    pub subcommand: BodhiCommand,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, StructOpt)]
pub enum BodhiCommand {
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
        karma: Option<Karma>,
    },
    /// Query bodhi for information about a compose
    ComposeInfo {
        /// release string
        release: FedoraRelease,
        /// request string ("stable" or "testing")
        request: ComposeRequest,
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<Format>,
    },
    /// Query bodhi for running composes
    ComposeList {
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<Format>,
    },
    /// Create a new buildroot override
    CreateOverride {
        /// NVR of the override
        nvr: String,
        /// duration (in days) it should be active
        #[structopt(long)]
        duration: u32,
        /// publicly visible notes
        #[structopt(long)]
        notes: String,
    },
    /// Create a new buildroot override for builds from an existing update
    CreateUpdateOverride {
        /// alias of the update (i.e. "FEDORA-2022-XXXXXXXXXX")
        alias: String,
        /// duration (in days) it should be active
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
        /// Require bug feedback for karma to count
        #[structopt(long)]
        require_bugs: Option<bool>,
        /// Require test case feedback for karma to count
        #[structopt(long)]
        require_testcases: Option<bool>,
        /// List of required gating tests
        #[structopt(long)]
        requirements: Option<Vec<String>>,
        /// Update severity
        #[structopt(long)]
        severity: Option<UpdateSeverity>,
        /// Days until it can be pushed to stable
        #[structopt(long)]
        stable_days: Option<u32>,
        /// Karma until it can be pushed to stable
        #[structopt(long)]
        stable_karma: Option<i32>,
        /// Logout / reboot suggestion
        #[structopt(long)]
        suggestion: Option<UpdateSuggestion>,
        /// Karma until it will be unpushed
        #[structopt(long)]
        unstable_karma: Option<i32>,
        /// Type of the update
        #[structopt(long, name = "type")]
        update_type: Option<UpdateType>,
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
        /// List of required gating tests
        #[structopt(long)]
        requirements: Option<Vec<String>>,
        /// Update severity
        #[structopt(long)]
        severity: Option<UpdateSeverity>,
        /// Days until it can be pushed to stable
        #[structopt(long)]
        stable_days: Option<u32>,
        /// Karma until it can be pushed to stable
        #[structopt(long)]
        stable_karma: Option<i32>,
        /// Logout / reboot suggestion
        #[structopt(long)]
        suggestion: Option<UpdateSuggestion>,
        /// Karma until it will be unpushed
        #[structopt(long)]
        unstable_karma: Option<i32>,
        /// Type of the update
        #[structopt(long, name = "type")]
        update_type: Option<UpdateType>,
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
        format: Option<Format>,
        /// Query for this release / these releases
        #[structopt(long)]
        releases: Option<Vec<FedoraRelease>>,
        /// Query for overrides submitted by these users
        #[structopt(long)]
        users: Option<Vec<String>>,
        /// Force long-running queries
        #[structopt(long, short)]
        force: bool,
    },
    /// Query bodhi for updates
    QueryUpdates {
        /// update with this alias
        #[structopt(long)]
        alias: Option<String>,
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
        content_type: Option<ContentType>,
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<Format>,
        /// locked updates
        #[structopt(long)]
        locked: Option<bool>,
        /// updates modified before this date
        #[structopt(long)]
        modified_before: Option<BodhiDate>,
        /// updates modified after this date
        #[structopt(long)]
        modified_since: Option<BodhiDate>,
        /// updates for these packages
        #[structopt(long)]
        packages: Option<Vec<String>>,
        /// pushed updates
        #[structopt(long)]
        pushed: Option<bool>,
        /// updates pushed before this date
        #[structopt(long)]
        pushed_before: Option<BodhiDate>,
        /// updates pushed after this date
        #[structopt(long)]
        pushed_since: Option<BodhiDate>,
        /// updates for these releases
        #[structopt(long)]
        releases: Option<Vec<FedoraRelease>>,
        /// updates with this status request
        #[structopt(long)]
        request: Option<UpdateRequest>,
        /// updates with this severity
        #[structopt(long)]
        severity: Option<UpdateSeverity>,
        /// updates with this status
        #[structopt(long)]
        status: Option<UpdateStatus>,
        /// updates submitted before this date
        #[structopt(long)]
        submitted_before: Option<BodhiDate>,
        /// updates submitted after this date
        #[structopt(long)]
        submitted_since: Option<BodhiDate>,
        /// updates with logout / reboot suggestion
        #[structopt(long)]
        suggestion: Option<UpdateSuggestion>,
        /// updates with this type
        #[structopt(name = "type", long)]
        update_type: Option<UpdateType>,
        /// updates submitted by this user
        #[structopt(long)]
        users: Option<Vec<String>>,
        /// Force long-running queries
        #[structopt(long, short)]
        force: bool,
    },
    /// Query bodhi for information about a release
    ReleaseInfo {
        /// ID of the release
        release: String,
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<Format>,
    },
    /// Query bodhi for active releases
    ReleaseList {
        /// Output format (plain, JSON)
        #[structopt(long)]
        format: Option<Format>,
    },
    /// Submit an update status request
    UpdateRequest {
        /// ID of the update
        alias: String,
        /// (obsolete, revoke, stable, testing, unpush)
        request: UpdateRequest,
    },
    /// Waive an update's test results
    WaiveTests {
        /// ID of the update
        alias: String,
        /// comment submitted with the waiver
        comment: String,
        /// test results to be waived (default: empty / all)
        #[structopt(long)]
        tests: Option<Vec<String>>,
    },
}

impl BaseCommand {
    pub fn authenticated(&self) -> bool {
        use BodhiCommand::*;

        match self.subcommand {
            Comment { .. } => true,
            ComposeInfo { .. } => false,
            ComposeList { .. } => false,
            CreateOverride { .. } => true,
            CreateUpdateOverride { .. } => true,
            CreateUpdate { .. } => true,
            EditOverride { .. } => true,
            EditUpdate { .. } => true,
            ExpireOverride { .. } => true,
            QueryOverrides { .. } => false,
            QueryUpdates { .. } => false,
            ReleaseInfo { .. } => false,
            ReleaseList { .. } => false,
            UpdateRequest { .. } => true,
            WaiveTests { .. } => true,
        }
    }
}

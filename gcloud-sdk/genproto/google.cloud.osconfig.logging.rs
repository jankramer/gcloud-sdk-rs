#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchJobCompletedLog {
    /// The patch job name. For example:
    /// projects/PROJECT_ID/patchJobs/PATCH_JOB_ID
    #[prost(string, tag = "1")]
    pub patch_job: ::prost::alloc::string::String,
    /// The current state of the PatchJob.
    #[prost(enumeration = "patch_job_completed_log::State", tag = "2")]
    pub state: i32,
    /// Summary of instance details.
    #[prost(message, optional, tag = "3")]
    pub instance_details_summary: ::core::option::Option<
        patch_job_completed_log::InstanceDetailsSummary,
    >,
    /// If this patch job is a dry run, the agent will report that it has
    /// finished without running any updates on the VM.
    #[prost(bool, tag = "4")]
    pub dry_run: bool,
    /// If this patch job failed, this message will provide information about the
    /// failure.
    #[prost(string, tag = "5")]
    pub error_message: ::prost::alloc::string::String,
    /// Time this PatchJob was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Last time this PatchJob was updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `PatchJobCompletedLog`.
pub mod patch_job_completed_log {
    /// A summary of the current patch state across all instances this patch job
    /// affects. Contains counts of instances in different states. These states map
    /// to InstancePatchState. List patch job instance details to see the specific
    /// states of each instance.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceDetailsSummary {
        /// Number of instances pending patch job.
        #[prost(int64, tag = "1")]
        pub instances_pending: i64,
        /// Number of instances that are inactive.
        #[prost(int64, tag = "2")]
        pub instances_inactive: i64,
        /// Number of instances notified about patch job.
        #[prost(int64, tag = "3")]
        pub instances_notified: i64,
        /// Number of instances that have started.
        #[prost(int64, tag = "4")]
        pub instances_started: i64,
        /// Number of instances that are downloading patches.
        #[prost(int64, tag = "5")]
        pub instances_downloading_patches: i64,
        /// Number of instances that are applying patches.
        #[prost(int64, tag = "6")]
        pub instances_applying_patches: i64,
        /// Number of instances rebooting.
        #[prost(int64, tag = "7")]
        pub instances_rebooting: i64,
        /// Number of instances that have completed successfully.
        #[prost(int64, tag = "8")]
        pub instances_succeeded: i64,
        /// Number of instances that require reboot.
        #[prost(int64, tag = "9")]
        pub instances_succeeded_reboot_required: i64,
        /// Number of instances that failed.
        #[prost(int64, tag = "10")]
        pub instances_failed: i64,
        /// Number of instances that have acked and will start shortly.
        #[prost(int64, tag = "11")]
        pub instances_acked: i64,
        /// Number of instances that exceeded the time out while applying the patch.
        #[prost(int64, tag = "12")]
        pub instances_timed_out: i64,
        /// Number of instances that are running the pre-patch step.
        #[prost(int64, tag = "13")]
        pub instances_running_pre_patch_step: i64,
        /// Number of instances that are running the post-patch step.
        #[prost(int64, tag = "14")]
        pub instances_running_post_patch_step: i64,
    }
    /// Enumeration of the various states a patch job passes through as it
    /// executes.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// State must be specified.
        Unspecified = 0,
        /// The patch job was successfully initiated.
        Started = 1,
        /// The patch job is looking up instances to run the patch on.
        InstanceLookup = 2,
        /// Instances are being patched.
        Patching = 3,
        /// Patch job completed successfully.
        Succeeded = 4,
        /// Patch job completed but there were errors.
        CompletedWithErrors = 5,
        /// The patch job was canceled.
        Canceled = 6,
        /// The patch job has timed out.
        TimedOut = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Started => "STARTED",
                State::InstanceLookup => "INSTANCE_LOOKUP",
                State::Patching => "PATCHING",
                State::Succeeded => "SUCCEEDED",
                State::CompletedWithErrors => "COMPLETED_WITH_ERRORS",
                State::Canceled => "CANCELED",
                State::TimedOut => "TIMED_OUT",
            }
        }
    }
}

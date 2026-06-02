#[derive(Debug, Clone, Copy)]
pub struct PackManifest {
    pub id: &'static str,
    pub name: &'static str,
    pub status: &'static str,
    pub authority_summary: &'static str,
    pub default_ghosts: &'static [&'static str],
}

#[derive(Debug, Clone, Copy)]
pub struct ProfileManifest {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: &'static str,
    pub authority_summary: &'static str,
    pub capabilities: &'static [ProfileCapability],
    pub default_ghosts: &'static [&'static str],
}

#[derive(Debug, Clone, Copy)]
pub struct ProfileCapability {
    pub key: &'static str,
    pub state: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct SelectedPackProfile {
    pub pack_id: &'static str,
    pub profile_id: &'static str,
}

pub const DEFAULT_PACK_ID: &str = "santo-andre";
pub const DEFAULT_PROFILE_ID: &str = "local-offline";

const SANTO_ANDRE_GHOSTS: &[&str] = &[
    "remote-spine-unconfigured",
    "evidence-registry-unimplemented",
    "receipt-closure-unimplemented",
    "interactive-lab-surface-unimplemented",
    "llm-translator-unimplemented",
];

const PERSONAL_OFFLINE_GHOSTS: &[&str] = &[
    "passkey-checkpointing-unimplemented",
    "batch-signing-unimplemented",
    "personal-adapters-unimplemented",
    "selective-disclosure-unimplemented",
    "evidence-registry-unimplemented",
    "receipt-closure-unimplemented",
];

const LOCAL_OFFLINE_GHOSTS: &[&str] = &[
    "remote-spine-unconfigured",
    "evidence-registry-unimplemented",
    "receipt-closure-unimplemented",
    "interactive-lab-surface-unimplemented",
    "llm-translator-unimplemented",
];

const SUPABASE_GHOSTS: &[&str] = &[
    "supabase-ingest-unconfigured",
    "supabase-env-unconfigured",
    "remote-spine-unconfigured",
    "evidence-registry-unimplemented",
    "receipt-closure-unimplemented",
];

const LOCAL_OFFLINE_CAPABILITIES: &[ProfileCapability] = &[
    ProfileCapability {
        key: "local_lab_home",
        state: "available",
    },
    ProfileCapability {
        key: "act_validation",
        state: "available",
    },
    ProfileCapability {
        key: "candidate_capture",
        state: "available",
    },
    ProfileCapability {
        key: "ghost_listing",
        state: "available",
    },
    ProfileCapability {
        key: "daily_state_report",
        state: "available",
    },
    ProfileCapability {
        key: "remote_spine",
        state: "unavailable/ghost",
    },
    ProfileCapability {
        key: "evidence_registry",
        state: "unavailable/ghost",
    },
    ProfileCapability {
        key: "receipt_closure",
        state: "unavailable/ghost",
    },
    ProfileCapability {
        key: "llm_translator",
        state: "unavailable/ghost",
    },
    ProfileCapability {
        key: "interactive_lab_surface",
        state: "unavailable/ghost",
    },
];

const SUPABASE_CAPABILITIES: &[ProfileCapability] = &[
    ProfileCapability {
        key: "remote_spine",
        state: "implemented_when_configured",
    },
    ProfileCapability {
        key: "act_validation",
        state: "available",
    },
    ProfileCapability {
        key: "candidate_capture",
        state: "available",
    },
    ProfileCapability {
        key: "local_lab_home",
        state: "available",
    },
    ProfileCapability {
        key: "supabase_ingest",
        state: "implemented_when_configured",
    },
    ProfileCapability {
        key: "supabase_env_verification",
        state: "available",
    },
    ProfileCapability {
        key: "evidence_registry",
        state: "unimplemented/ghost",
    },
    ProfileCapability {
        key: "receipt_closure",
        state: "unimplemented/ghost",
    },
];

const PACKS: &[PackManifest] = &[
    PackManifest {
        id: "santo-andre",
        name: "Santo André",
        status: "recommended_reference_practice",
        authority_summary: "local practice; not canon amendment; not official pack",
        default_ghosts: SANTO_ANDRE_GHOSTS,
    },
    PackManifest {
        id: "personal-offline",
        name: "Personal Offline",
        status: "private_local_practice",
        authority_summary: "local practice; not canon amendment; not official pack",
        default_ghosts: PERSONAL_OFFLINE_GHOSTS,
    },
];

const PROFILES: &[ProfileManifest] = &[
    ProfileManifest {
        id: "local-offline",
        name: "Local Offline",
        kind: "local_workspace_profile",
        authority_summary: "local workspace only; not official spine; not receipt",
        capabilities: LOCAL_OFFLINE_CAPABILITIES,
        default_ghosts: LOCAL_OFFLINE_GHOSTS,
    },
    ProfileManifest {
        id: "supabase",
        name: "Supabase Online Spine",
        kind: "online_spine_profile",
        authority_summary:
            "declared online spine profile; requires configuration; not universal canon",
        capabilities: SUPABASE_CAPABILITIES,
        default_ghosts: SUPABASE_GHOSTS,
    },
];

pub fn known_pack(id: &str) -> Option<&'static PackManifest> {
    PACKS.iter().find(|pack| pack.id == id)
}

pub fn known_profile(id: &str) -> Option<&'static ProfileManifest> {
    PROFILES.iter().find(|profile| profile.id == id)
}

pub fn default_selection() -> SelectedPackProfile {
    SelectedPackProfile {
        pack_id: DEFAULT_PACK_ID,
        profile_id: DEFAULT_PROFILE_ID,
    }
}

pub fn validate_selection(pack_id: &str, profile_id: &str) -> Result<SelectedPackProfile, String> {
    let pack = known_pack(pack_id).ok_or_else(|| format!("unknown pack: {pack_id}"))?;
    let profile =
        known_profile(profile_id).ok_or_else(|| format!("unknown profile: {profile_id}"))?;
    Ok(SelectedPackProfile {
        pack_id: pack.id,
        profile_id: profile.id,
    })
}

pub fn selection_ghosts(selection: &SelectedPackProfile) -> Vec<&'static str> {
    let mut ghosts = Vec::new();
    if let Some(pack) = known_pack(selection.pack_id) {
        append_unique(&mut ghosts, pack.default_ghosts);
    }
    if let Some(profile) = known_profile(selection.profile_id) {
        append_unique(&mut ghosts, profile.default_ghosts);
    }
    append_unique(&mut ghosts, &["yaml-act-parser-unimplemented"]);
    ghosts
}

fn append_unique(target: &mut Vec<&'static str>, values: &'static [&'static str]) {
    for value in values {
        if !target.iter().any(|existing| existing == value) {
            target.push(value);
        }
    }
}

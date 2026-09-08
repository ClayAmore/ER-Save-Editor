pub mod slot_view_model {
    // One per slot: vm.rs writes it only for slots with a real character, so
    // the UI reads `active` to decide which slots to show.
    #[derive(Clone, Default)]
    pub struct ProfileSummaryViewModel {
        pub active: bool,
    }
}

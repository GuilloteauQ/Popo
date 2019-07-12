//! Representation of the sections

#[derive(Clone, Debug, PartialEq)]
pub enum Section {
    /// Title of the page
    Title,
    /// Goals of the day
    Goals,
    /// What should be done for the next day
    TODOTomorrow,
    /// A documentation of what have been done today
    DoneToday,
}

impl Section {
    /// Returns the classical order of sections
    pub fn default_order() -> Vec<Self> {
        vec![
            Section::Title,
            Section::Goals,
            Section::TODOTomorrow,
            Section::DoneToday,
        ]
    }
}

impl ToString for Section {
    fn to_string(&self) -> String {
        String::from(match &self {
            Section::Title => "#",
            Section::Goals => "## Goals of the Day\n",
            Section::TODOTomorrow => "## TODO Tomorrow\n",
            Section::DoneToday => "## Done Today\n",
        })
    }
}

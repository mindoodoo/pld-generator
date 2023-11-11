use serde::{self, Deserialize};

/// Structure representing one card on the project
#[derive(Debug)]
pub struct ProjectCard {
    pub name: String,
    pub content: String,
    pub section: String,
    pub sub_section: String,
    pub working_days: f32
}

// This deserializer implementation allows for deserializing a given card (aka Node)
// while ditching the unnecessary nesting from the response
impl<'de> Deserialize<'de> for ProjectCard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Content {
            title: String,
            body: String 
        }

        #[derive(Deserialize)]
        struct WorkingDays {
            number: f32
        }

        #[derive(Deserialize)]
        struct Section {
            name: String
        }

        #[derive(Deserialize)]
        struct SubSection {
            name: String
        }

        #[derive(Deserialize)]
        struct Node {
            content: Content,
            working_days: WorkingDays,
            section: Option<Section>,
            sub_section: Option<SubSection>
        }

        let helper = Node::deserialize(deserializer)?;

        Ok(ProjectCard {
            name: helper.content.title,
            content: helper.content.body,
            section: if let Some(section) = helper.section {
                section.name
            } else { "".to_string() },
            working_days: helper.working_days.number,
            sub_section: if let Some(subsection) = helper.sub_section {
                subsection.name
            } else { "".to_string() }
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct PagingInfo {
    #[serde(rename = "endCursor")]
    pub end_cursor: String
}

#[derive(Deserialize, Debug)]
pub struct ProjectItems {
    #[serde(rename = "totalCount")]
    pub total_count: usize,
    pub nodes: Vec<ProjectCard>,
    #[serde(rename = "pageInfo")]
    pub paging_info: PagingInfo
}
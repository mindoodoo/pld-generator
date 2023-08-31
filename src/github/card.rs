use serde::{self, Deserialize};

/// Structure representing one card on the project
#[derive(Debug)]
pub struct ProjectCard {
    pub name: String,
    pub content: String,
    pub section: String,
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
            #[serde(rename = "bodyText")]
            body_text: String 
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
        struct Node {
            content: Content,
            working_days: WorkingDays,
            section: Section
        }

        let helper = Node::deserialize(deserializer)?;

        Ok(ProjectCard {
            name: helper.content.title,
            content: helper.content.body_text,
            section: helper.section.name,
            working_days: helper.working_days.number
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
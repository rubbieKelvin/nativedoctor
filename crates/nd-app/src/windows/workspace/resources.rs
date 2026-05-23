use gpui::SharedString;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Request,
    Sequence,
    Environment,
    Folder,
}

impl ResourceType {
    pub fn prefix(&self) -> &'static str {
        return match self {
            ResourceType::Request => "request:",
            ResourceType::Sequence => "sequence:",
            ResourceType::Environment => "env:",
            ResourceType::Folder => "folder:",
        };
    }

    pub fn make_id(&self, name: &str) -> SharedString {
        return SharedString::from(format!("{}{}", self.prefix(), name));
    }

    pub fn from_id(id: &str) -> Option<Self> {
        if id.starts_with("folder:") {
            return Some(ResourceType::Folder);
        }
        if id.starts_with("env:") {
            return Some(ResourceType::Environment);
        }
        if id.starts_with("sequence:") {
            return Some(ResourceType::Sequence);
        }
        if id.starts_with("request:") {
            return Some(ResourceType::Request);
        }
        return None;
    }

    pub fn to_tab_kind(&self) -> super::TabKind {
        return match self {
            ResourceType::Request => super::TabKind::Request,
            ResourceType::Sequence => super::TabKind::Sequence,
            ResourceType::Environment => super::TabKind::Environment,
            ResourceType::Folder => super::TabKind::Request,
        };
    }
}

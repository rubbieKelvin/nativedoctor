//! The collection as a tree of folders and requests.

use gpui_kit::component::tree::TreeItem;
use nd_core::collection::TreeNode;
use nd_core::RequestFile;

/// Request label: its name, or the file name without its suffix.
fn label_of(path: &std::path::Path) -> String {
    return RequestFile::load(path).map_or_else(
        |_| RequestFile::default().display_name(path),
        |file| file.display_name(path),
    );
}

/// Items for the tree. An item's id is its path, so a click can say which file it means.
pub fn tree_items(nodes: &[TreeNode]) -> Vec<TreeItem> {
    return nodes
        .iter()
        .map(|node| match node {
            TreeNode::Folder {
                name,
                path,
                children,
            } => TreeItem::new(path.display().to_string(), name.clone())
                .expanded(true)
                .children(tree_items(children)),
            TreeNode::Request { path } => {
                TreeItem::new(path.display().to_string(), label_of(path))
            }
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use nd_core::Collection;

    #[test]
    fn ids_are_paths_and_labels_are_request_names() {
        let dir = tempfile::tempdir().unwrap();
        let collection = Collection::init(dir.path(), None).unwrap();
        std::fs::create_dir_all(dir.path().join("users")).unwrap();
        std::fs::write(
            dir.path().join("users/get.request.yaml"),
            "name: Get user\nhttp: { url: x }\n",
        )
        .unwrap();

        let items = tree_items(&collection.tree().unwrap());
        assert_eq!(items.len(), 2);
        let users = &items[1];
        assert!(users.is_folder() && users.is_expanded());
        let request = &users.children[0];
        assert!(request.id.ends_with("users/get.request.yaml"));
        assert_eq!(request.label.as_ref(), "Get user");
        assert_eq!(items[0].label.as_ref(), "Ping");
    }
}
